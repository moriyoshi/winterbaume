//! Standalone HTTP server for winterbaume mock AWS services.
//!
//! Starts an HTTP server that handles AWS SDK requests, allowing
//! non-Rust clients or integration tests to use winterbaume as a local
//! AWS endpoint.
//!
//! # Usage
//!
//! ```text
//! winterbaume-server [OPTIONS]
//! ```
//!
//! # Configuration precedence
//!
//! CLI flags > environment variables > config file > built-in defaults.
//!
//! # Config file
//!
//! Pass `--config FILE` (or set `WB_CONFIG`) to load a TOML file:
//!
//! ```toml
//! host       = "127.0.0.1"
//! port       = 5555
//! account_id = "123456789012"
//! region     = "us-east-1"
//! tfstate    = "/path/to/terraform.tfstate"
//!
//! [backends]
//! # Requires: --features backend-sqs-redis
//! sqs = "redis://127.0.0.1/"
//! # Requires: --features backend-dynamodb-redis
//! dynamodb = "redis://127.0.0.1/"
//! # Requires: --features backend-vfs-fs
//! vfs_dir = "/var/lib/winterbaume/blobs"
//! # Requires: --features backend-sqlengine-duckdb (or -bundled).
//! # Use ":memory:" for an in-memory DuckDB or a filesystem path to persist.
//! sqlengine_duckdb = ":memory:"
//! ```
//!
//! # Environment variables
//!
//! | Variable              | Equivalent flag         |
//! |-----------------------|-------------------------|
//! | `WB_HOST`             | `--host`                |
//! | `WB_PORT`             | `--port`                |
//! | `WB_ACCOUNT_ID`       | `--account-id`          |
//! | `WB_REGION`           | `--region`              |
//! | `WB_TFSTATE`          | `--tfstate`             |
//! | `WB_CONFIG`           | `--config`              |
//! | `WB_SQS_BACKEND`      | `--sqs-backend`         |
//! | `WB_DYNAMODB_BACKEND` | `--dynamodb-backend`    |
//! | `WB_VFS_DIR`          | `--vfs-dir`             |
//! | `WB_SQLENGINE_DUCKDB` | `--sqlengine-duckdb`    |
//!
//! Default: listens on 127.0.0.1:5555

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use bytes::Bytes;
use clap::{CommandFactory, FromArgMatches, Parser};
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use regex::Regex;
use serde::Deserialize;
use tokio::net::TcpListener;
use winterbaume_core::service::{MockRequest, MockResponse, MockService};

// ---------------------------------------------------------------------------
// Config file schema
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize)]
struct ConfigFile {
    host: Option<String>,
    port: Option<u16>,
    account_id: Option<String>,
    region: Option<String>,
    tfstate: Option<String>,
    backends: Option<BackendsConfig>,
}

#[derive(Debug, Default, Deserialize)]
struct BackendsConfig {
    /// Redis URL for SQS backend (requires `backend-sqs-redis` feature).
    sqs: Option<String>,
    /// Redis URL for DynamoDB backend (requires `backend-dynamodb-redis` feature).
    dynamodb: Option<String>,
    /// Directory for filesystem VFS (requires `backend-vfs-fs` feature).
    vfs_dir: Option<String>,
    /// DuckDB path / `:memory:` for the Athena / Redshift Data SQL engine
    /// (requires `backend-sqlengine-duckdb` or `backend-sqlengine-duckdb-bundled`).
    sqlengine_duckdb: Option<String>,
}

// ---------------------------------------------------------------------------
// Merged runtime options
// ---------------------------------------------------------------------------

/// All resolved server configuration after merging CLI > env > config > defaults.
struct ServerOptions {
    host: String,
    port: u16,
    account_id: String,
    default_region: String,
    tfstate_path: Option<String>,
    backends: BackendOptions,
}

/// Backend selection options.  Fields are always present; features gate whether
/// a non-None value is acted upon.
struct BackendOptions {
    /// Redis URL for the SQS backend.
    sqs_backend: Option<String>,
    /// Redis URL for the DynamoDB backend.
    dynamodb_backend: Option<String>,
    /// Filesystem directory for VFS blob storage.
    vfs_dir: Option<String>,
    /// DuckDB path / `:memory:` for the Athena and Redshift Data services.
    sqlengine_duckdb: Option<String>,
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

struct ServiceRoute {
    pattern: Regex,
    service: Arc<dyn MockService>,
}

struct Router {
    routes: Vec<ServiceRoute>,
}

impl Router {
    fn new(services: Vec<Arc<dyn MockService>>) -> Self {
        let mut routes = Vec::new();
        for service in services {
            for pattern in service.url_patterns() {
                routes.push(ServiceRoute {
                    pattern: Regex::new(pattern)
                        .unwrap_or_else(|e| panic!("Invalid URL pattern '{}': {}", pattern, e)),
                    service: Arc::clone(&service),
                });
            }
        }
        Self { routes }
    }

    fn find_service(&self, uri: &str, headers: &http::HeaderMap) -> Option<Arc<dyn MockService>> {
        // Try URL pattern matching first
        for route in &self.routes {
            if route.pattern.is_match(uri) {
                return Some(Arc::clone(&route.service));
            }
        }

        // Fall back to host-based service extraction
        if let Some(service_name) = winterbaume_core::auth::extract_service_from_uri(uri) {
            if let Some(route) = self
                .routes
                .iter()
                .find(|r| r.service.service_name() == service_name)
            {
                return Some(Arc::clone(&route.service));
            }
        }

        // Fall back to Authorization header SigV4 credential scope
        if let Some(service_name) = winterbaume_core::auth::extract_service_from_headers(headers) {
            if let Some(route) = self
                .routes
                .iter()
                .find(|r| r.service.service_name() == service_name)
            {
                return Some(Arc::clone(&route.service));
            }
        }

        None
    }
}

// ---------------------------------------------------------------------------
// Request handling
// ---------------------------------------------------------------------------

async fn handle_request(
    router: Arc<Router>,
    remote_addr: SocketAddr,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let start = Instant::now();
    let method = req.method().to_string();

    // Reconstruct full URI with host for service routing
    let host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("localhost");
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|pq| pq.to_string())
        .unwrap_or_else(|| "/".to_string());
    let full_uri = format!("https://{host}{path_and_query}");

    // Copy headers
    let mut headers = http::HeaderMap::new();
    for (name, value) in req.headers() {
        headers.insert(name.clone(), value.clone());
    }

    // Read body
    let body_bytes = req
        .into_body()
        .collect()
        .await
        .map(|c| c.to_bytes())
        .unwrap_or_default();

    let mock_request = MockRequest {
        method: method.clone(),
        uri: full_uri.clone(),
        headers,
        body: body_bytes,
    };

    // Route to service
    let mock_response = match router.find_service(&full_uri, &mock_request.headers) {
        Some(service) => service.handle(mock_request).await,
        None => MockResponse::json(
            404,
            format!(
                r#"{{"__type":"UnknownService","message":"No service registered for: {}"}}"#,
                full_uri
            ),
        ),
    };

    // Build hyper response
    let mut response = Response::builder().status(mock_response.status);
    for (name, value) in &mock_response.headers {
        response = response.header(name, value);
    }

    let resp = response
        .body(Full::new(mock_response.body))
        .unwrap_or_else(|_| {
            Response::builder()
                .status(500)
                .body(Full::new(Bytes::from("Internal Server Error")))
                .unwrap()
        });

    let elapsed = start.elapsed();
    tracing::info!(
        remote_addr = %remote_addr,
        method = %method,
        path = path_and_query,
        status = resp.status().as_u16(),
        elapsed_ms = format_args!("{:.3}", elapsed.as_secs_f64() * 1000.0),
        "request",
    );

    Ok(resp)
}

// ---------------------------------------------------------------------------
// Service registration
// ---------------------------------------------------------------------------

/// References to services that support terraform state injection.
struct InjectableServices {
    accessanalyzer: Arc<winterbaume_accessanalyzer::AccessAnalyzerService>,
    acm: Arc<winterbaume_acm::AcmService>,
    apigatewayv2: Arc<winterbaume_apigatewayv2::ApiGatewayV2Service>,
    appfabric: Arc<winterbaume_appfabric::AppFabricService>,
    appflow: Arc<winterbaume_appflow::AppFlowService>,
    applicationcostprofiler:
        Arc<winterbaume_applicationcostprofiler::ApplicationCostProfilerService>,
    appsync: Arc<winterbaume_appsync::AppSyncService>,
    amplify: Arc<winterbaume_amplify::AmplifyService>,
    apprunner: Arc<winterbaume_apprunner::AppRunnerService>,
    athena: Arc<winterbaume_athena::AthenaService>,
    auditmanager: Arc<winterbaume_auditmanager::AuditManagerService>,
    backup: Arc<winterbaume_backup::BackupService>,
    chatbot: Arc<winterbaume_chatbot::ChatbotService>,
    cloudformation: Arc<winterbaume_cloudformation::CloudFormationService>,
    cloudfront: Arc<winterbaume_cloudfront::CloudFrontService>,
    cloudwatch: Arc<winterbaume_cloudwatch::CloudWatchService>,
    codebuild: Arc<winterbaume_codebuild::CodeBuildService>,
    codeartifact: Arc<winterbaume_codeartifact::CodeArtifactService>,
    codecommit: Arc<winterbaume_codecommit::CodeCommitService>,
    codedeploy: Arc<winterbaume_codedeploy::CodeDeployService>,
    codepipeline: Arc<winterbaume_codepipeline::CodePipelineService>,
    cognitoidp: Arc<winterbaume_cognitoidentityprovider::CognitoIdentityProviderService>,
    config: Arc<winterbaume_config::ConfigService>,
    dynamodb: Arc<winterbaume_dynamodb::DynamoDbService>,
    ecr: Arc<winterbaume_ecr::EcrService>,
    ecs: Arc<winterbaume_ecs::EcsService>,
    efs: Arc<winterbaume_efs::EfsService>,
    eks: Arc<winterbaume_eks::EksService>,
    elasticache: Arc<winterbaume_elasticache::ElastiCacheService>,
    elbv2: Arc<winterbaume_elasticloadbalancingv2::ElasticLoadBalancingV2Service>,
    emr: Arc<winterbaume_emr::EmrService>,
    events: Arc<winterbaume_eventbridge::EventBridgeService>,
    firehose: Arc<winterbaume_firehose::FirehoseService>,
    fis: Arc<winterbaume_fis::FisService>,
    glue: Arc<winterbaume_glue::GlueService>,
    iam: Arc<winterbaume_iam::IamService>,
    kafka: Arc<winterbaume_kafka::KafkaService>,
    kinesis: Arc<winterbaume_kinesis::KinesisService>,
    kms: Arc<winterbaume_kms::KmsService>,
    lambda: Arc<winterbaume_lambda::LambdaService>,
    logs: Arc<winterbaume_cloudwatchlogs::CloudWatchLogsService>,
    opensearch: Arc<winterbaume_opensearch::OpenSearchService>,
    opensearchserverless: Arc<winterbaume_opensearchserverless::OpenSearchServerlessService>,
    redshift: Arc<winterbaume_redshift::RedshiftService>,
    route53: Arc<winterbaume_route53::Route53Service>,
    s3: Arc<winterbaume_s3::S3Service>,
    sagemaker: Arc<winterbaume_sagemaker::SageMakerService>,
    scheduler: Arc<winterbaume_scheduler::SchedulerService>,
    secretsmanager: Arc<winterbaume_secretsmanager::SecretsManagerService>,
    ses: Arc<winterbaume_sesv2::SesV2Service>,
    sesv1: Arc<winterbaume_ses::SesService>,
    sns: Arc<winterbaume_sns::SnsService>,
    sqs: Arc<winterbaume_sqs::SqsService>,
    ssm: Arc<winterbaume_ssm::SsmService>,
    ssoadmin: Arc<winterbaume_ssoadmin::SsoAdminService>,
    stepfunctions: Arc<winterbaume_sfn::SfnService>,
    wafv2: Arc<winterbaume_wafv2::WafV2Service>,
    workspaces: Arc<winterbaume_workspaces::WorkSpacesService>,
    account: Arc<winterbaume_account::AccountService>,
    acmpca: Arc<winterbaume_acmpca::AcmPcaService>,
    amp: Arc<winterbaume_amp::AmpService>,
    apigateway: Arc<winterbaume_apigateway::ApiGatewayService>,
    appconfig: Arc<winterbaume_appconfig::AppConfigService>,
    applicationautoscaling: Arc<winterbaume_applicationautoscaling::ApplicationAutoScalingService>,
    appmesh: Arc<winterbaume_appmesh::AppMeshService>,
    autoscaling: Arc<winterbaume_autoscaling::AutoScalingService>,
    batch: Arc<winterbaume_batch::BatchService>,
    bedrock: Arc<winterbaume_bedrock::BedrockService>,
    bedrockagent: Arc<winterbaume_bedrockagent::BedrockAgentService>,
    budgets: Arc<winterbaume_budgets::BudgetsService>,
    cloudhsmv2: Arc<winterbaume_cloudhsmv2::CloudHsmV2Service>,
    cloudtrail: Arc<winterbaume_cloudtrail::CloudTrailService>,
    cognitoidentity: Arc<winterbaume_cognitoidentity::CognitoIdentityService>,
    comprehend: Arc<winterbaume_comprehend::ComprehendService>,
    connect: Arc<winterbaume_connect::ConnectService>,
    costexplorer: Arc<winterbaume_costexplorer::CostExplorerService>,
    datapipeline: Arc<winterbaume_datapipeline::DataPipelineService>,
    datasync: Arc<winterbaume_datasync::DataSyncService>,
    directconnect: Arc<winterbaume_directconnect::DirectConnectService>,
    directory: Arc<winterbaume_directory::DirectoryService>,
    dms: Arc<winterbaume_databasemigration::DatabaseMigrationService>,
    dsql: Arc<winterbaume_dsql::DsqlService>,
    ec2: Arc<winterbaume_ec2::Ec2Service>,
    elasticloadbalancing: Arc<winterbaume_elasticloadbalancing::ElasticLoadBalancingService>,
    emrcontainers: Arc<winterbaume_emrcontainers::EmrContainersService>,
    emrserverless: Arc<winterbaume_emrserverless::EmrServerlessService>,
    glacier: Arc<winterbaume_glacier::GlacierService>,
    guardduty: Arc<winterbaume_guardduty::GuardDutyService>,
    identitystore: Arc<winterbaume_identitystore::IdentityStoreService>,
    inspector2: Arc<winterbaume_inspector2::Inspector2Service>,
    iot: Arc<winterbaume_iot::IotService>,
    ivs: Arc<winterbaume_ivs::IvsService>,
    kinesisanalyticsv2: Arc<winterbaume_kinesisanalyticsv2::KinesisAnalyticsV2Service>,
    kinesisvideo: Arc<winterbaume_kinesisvideo::KinesisVideoService>,
    lakeformation: Arc<winterbaume_lakeformation::LakeFormationService>,
    lexmodelsv2: Arc<winterbaume_lexmodelsv2::LexModelsV2Service>,
    macie2: Arc<winterbaume_macie2::Macie2Service>,
    medialive: Arc<winterbaume_medialive::MediaLiveService>,
    mediapackage: Arc<winterbaume_mediapackage::MediaPackageService>,
    mediapackagev2: Arc<winterbaume_mediapackagev2::MediaPackageV2Service>,
    mediastore: Arc<winterbaume_mediastore::MediaStoreService>,
    mq: Arc<winterbaume_mq::MqService>,
    networkfirewall: Arc<winterbaume_networkfirewall::NetworkFirewallService>,
    networkmanager: Arc<winterbaume_networkmanager::NetworkManagerService>,
    organizations: Arc<winterbaume_organizations::OrganizationsService>,
    osis: Arc<winterbaume_osis::OsisService>,
    outposts: Arc<winterbaume_outposts::OutpostsService>,
    keyspaces: Arc<winterbaume_keyspaces::KeyspacesService>,
    rolesanywhere: Arc<winterbaume_rolesanywhere::RolesAnywhereService>,
    fsx: Arc<winterbaume_fsx::FsxService>,
    pinpoint: Arc<winterbaume_pinpoint::PinpointService>,
    pipes: Arc<winterbaume_pipes::PipesService>,
    quicksight: Arc<winterbaume_quicksight::QuickSightService>,
    ram: Arc<winterbaume_ram::RamService>,
    rds: Arc<winterbaume_rds::RdsService>,
    rekognition: Arc<winterbaume_rekognition::RekognitionService>,
    resiliencehub: Arc<winterbaume_resiliencehub::ResilienceHubService>,
    resourcegroups: Arc<winterbaume_resourcegroups::ResourceGroupsService>,
    route53domains: Arc<winterbaume_route53domains::Route53DomainsService>,
    route53resolver: Arc<winterbaume_route53resolver::Route53ResolverService>,
    s3control: Arc<winterbaume_s3control::S3ControlService>,
    s3tables: Arc<winterbaume_s3tables::S3TablesService>,
    securityhub: Arc<winterbaume_securityhub::SecurityHubService>,
    servicecatalog: Arc<winterbaume_servicecatalog::ServiceCatalogService>,
    servicecatalogappregistry:
        Arc<winterbaume_servicecatalogappregistry::ServiceCatalogAppRegistryService>,
    servicediscovery: Arc<winterbaume_servicediscovery::ServiceDiscoveryService>,
    servicequotas: Arc<winterbaume_servicequotas::ServiceQuotasService>,
    shield: Arc<winterbaume_shield::ShieldService>,
    signer: Arc<winterbaume_signer::SignerService>,
    simpledbv2: Arc<winterbaume_simpledbv2::SimpleDbV2Service>,
    swf: Arc<winterbaume_swf::SwfService>,
    synthetics: Arc<winterbaume_synthetics::SyntheticsService>,
    timestreaminfluxdb: Arc<winterbaume_timestreaminfluxdb::TimestreamInfluxDbService>,
    timestreamquery: Arc<winterbaume_timestreamquery::TimestreamQueryService>,
    timestreamwrite: Arc<winterbaume_timestreamwrite::TimestreamWriteService>,
    transcribe: Arc<winterbaume_transcribe::TranscribeService>,
    transfer: Arc<winterbaume_transfer::TransferService>,
    vpclattice: Arc<winterbaume_vpclattice::VpcLatticeService>,
    xray: Arc<winterbaume_xray::XRayService>,
}

async fn register_all_services(
    opts: &BackendOptions,
) -> Result<(Vec<Arc<dyn MockService>>, InjectableServices), Box<dyn std::error::Error + Send + Sync>>
{
    // --- VFS (S3, Glacier, EBS blob storage) --------------------------------

    // Build a shared VFS when a filesystem directory is configured.
    // Without the `backend-vfs-fs` feature the option is accepted but ignored
    // with a warning, keeping the binary forward-compatible with config files
    // written for feature-full builds.
    let shared_vfs: Option<Arc<dyn winterbaume_core::Vfs>> = match &opts.vfs_dir {
        None => None,
        Some(dir) => {
            #[cfg(feature = "backend-vfs-fs")]
            {
                match winterbaume_core::FsVfs::new(dir) {
                    Ok(vfs) => {
                        tracing::info!("VFS: using FsVfs at '{}'", dir);
                        Some(Arc::new(vfs) as Arc<dyn winterbaume_core::Vfs>)
                    }
                    Err(e) => {
                        eprintln!("error: failed to create FsVfs at '{}': {}", dir, e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(feature = "backend-vfs-fs"))]
            {
                tracing::warn!(
                    "vfs_dir / --vfs-dir / WB_VFS_DIR is set to '{}' but this binary was \
                     compiled without the 'backend-vfs-fs' feature; using in-memory VFS",
                    dir
                );
                None
            }
        }
    };

    // --- S3 -----------------------------------------------------------------

    let s3: Arc<winterbaume_s3::S3Service> = Arc::new(match &shared_vfs {
        Some(vfs) => winterbaume_s3::S3Service::recover_with_vfs(Arc::clone(vfs)).await?,
        None => winterbaume_s3::S3Service::new(),
    });

    // --- IAM ----------------------------------------------------------------

    let iam = Arc::new(winterbaume_iam::IamService::new());

    // --- SQS ----------------------------------------------------------------

    let sqs: Arc<winterbaume_sqs::SqsService> = match &opts.sqs_backend {
        None => Arc::new(winterbaume_sqs::SqsService::new()),
        Some(url) => {
            #[cfg(feature = "backend-sqs-redis")]
            {
                match winterbaume_sqs_redis::RedisSqsBackend::from_connection_info(url.as_ref())
                    .await
                {
                    Ok(backend) => {
                        tracing::info!("SQS: using Redis backend at '{}'", url);
                        Arc::new(winterbaume_sqs::SqsService::with_backend(Arc::new(backend)))
                    }
                    Err(e) => {
                        eprintln!(
                            "error: failed to connect SQS Redis backend '{}': {}",
                            url, e
                        );
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(feature = "backend-sqs-redis"))]
            {
                tracing::warn!(
                    "sqs / --sqs-backend / WB_SQS_BACKEND is set to '{}' but this binary was \
                     compiled without the 'backend-sqs-redis' feature; using in-memory backend",
                    url
                );
                Arc::new(winterbaume_sqs::SqsService::new())
            }
        }
    };

    // --- DynamoDB -----------------------------------------------------------

    let dynamodb: Arc<winterbaume_dynamodb::DynamoDbService> = match &opts.dynamodb_backend {
        None => Arc::new(winterbaume_dynamodb::DynamoDbService::new()),
        Some(url) => {
            #[cfg(feature = "backend-dynamodb-redis")]
            {
                match winterbaume_dynamodb_redis::RedisDynamoDbBackend::from_connection_info(
                    url.as_ref(),
                )
                .await
                {
                    Ok(backend) => {
                        tracing::info!("DynamoDB: using Redis backend at '{}'", url);
                        Arc::new(winterbaume_dynamodb::DynamoDbService::with_backend(
                            Arc::new(backend),
                        ))
                    }
                    Err(e) => {
                        eprintln!(
                            "error: failed to connect DynamoDB Redis backend '{}': {}",
                            url, e
                        );
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(feature = "backend-dynamodb-redis"))]
            {
                tracing::warn!(
                    "dynamodb / --dynamodb-backend / WB_DYNAMODB_BACKEND is set to '{}' but \
                     this binary was compiled without the 'backend-dynamodb-redis' feature; \
                     using in-memory backend",
                    url
                );
                Arc::new(winterbaume_dynamodb::DynamoDbService::new())
            }
        }
    };

    // --- DuckDB SQL engine (Athena + Redshift Data) -------------------------

    // When configured + the feature is on, both services share one DuckDB
    // database so a CREATE TABLE through one service is visible to the other,
    // matching how a real Athena/Redshift Data Glue catalog would behave.
    #[cfg(feature = "backend-sqlengine-duckdb")]
    let duckdb_conn: Option<
        std::sync::Arc<std::sync::Mutex<winterbaume_sqlengine_duckdb::Connection>>,
    > = match &opts.sqlengine_duckdb {
        None => None,
        Some(path) => match winterbaume_sqlengine_duckdb::open_database(path) {
            Ok(conn) => {
                tracing::info!("SQL engine: using DuckDB at '{}'", path);
                Some(conn)
            }
            Err(e) => {
                eprintln!("error: failed to open DuckDB at '{}': {}", path, e);
                std::process::exit(1);
            }
        },
    };
    #[cfg(not(feature = "backend-sqlengine-duckdb"))]
    if let Some(path) = &opts.sqlengine_duckdb {
        tracing::warn!(
            "sqlengine_duckdb / --sqlengine-duckdb / WB_SQLENGINE_DUCKDB is set to '{}' but \
             this binary was compiled without the 'backend-sqlengine-duckdb' feature; using \
             in-memory SQL engine backend",
            path
        );
    }

    // --- Glacier & EBS (also use VFS) ---------------------------------------

    let glacier: Arc<winterbaume_glacier::GlacierService> = Arc::new(match &shared_vfs {
        Some(vfs) => winterbaume_glacier::GlacierService::with_vfs(Arc::clone(vfs)),
        None => winterbaume_glacier::GlacierService::new(),
    });

    let ebs: Arc<winterbaume_ebs::EbsService> = Arc::new(match &shared_vfs {
        Some(vfs) => winterbaume_ebs::EbsService::with_vfs(Arc::clone(vfs)),
        None => winterbaume_ebs::EbsService::new(),
    });

    // --- Services that also need injectable references ----------------------

    let accessanalyzer = Arc::new(winterbaume_accessanalyzer::AccessAnalyzerService::new());
    let acm = Arc::new(winterbaume_acm::AcmService::new());
    let apigateway = Arc::new(winterbaume_apigateway::ApiGatewayService::new());
    let apigatewaymanagementapi =
        Arc::new(winterbaume_apigatewaymanagement::ApiGatewayManagementService::new());
    let apigatewayv2 = Arc::new(winterbaume_apigatewayv2::ApiGatewayV2Service::new());
    let appfabric = Arc::new(winterbaume_appfabric::AppFabricService::new());
    let appflow = Arc::new(winterbaume_appflow::AppFlowService::new());
    let applicationcostprofiler =
        Arc::new(winterbaume_applicationcostprofiler::ApplicationCostProfilerService::new());
    let appsync = Arc::new(winterbaume_appsync::AppSyncService::new());
    let amplify = Arc::new(winterbaume_amplify::AmplifyService::new());
    let apprunner = Arc::new(winterbaume_apprunner::AppRunnerService::new());
    let athena = Arc::new({
        #[cfg(feature = "backend-sqlengine-duckdb")]
        {
            match &duckdb_conn {
                Some(conn) => winterbaume_athena::AthenaService::with_query_backend(Arc::new(
                    winterbaume_sqlengine_duckdb::DuckDbAthenaQueryBackend::new(Arc::clone(conn)),
                )),
                None => winterbaume_athena::AthenaService::new(),
            }
        }
        #[cfg(not(feature = "backend-sqlengine-duckdb"))]
        {
            winterbaume_athena::AthenaService::new()
        }
    });
    let auditmanager = Arc::new(winterbaume_auditmanager::AuditManagerService::new());
    let backup = Arc::new(winterbaume_backup::BackupService::new());
    let cloudformation = Arc::new(winterbaume_cloudformation::CloudFormationService::new());
    let cloudfront = Arc::new(winterbaume_cloudfront::CloudFrontService::new());
    let chatbot = Arc::new(winterbaume_chatbot::ChatbotService::new());
    let cloudwatch = Arc::new(winterbaume_cloudwatch::CloudWatchService::new());
    let codeartifact = Arc::new(winterbaume_codeartifact::CodeArtifactService::new());
    let codebuild = Arc::new(winterbaume_codebuild::CodeBuildService::new());
    let codecommit = Arc::new(winterbaume_codecommit::CodeCommitService::new());
    let codedeploy = Arc::new(winterbaume_codedeploy::CodeDeployService::new());
    let codepipeline = Arc::new(winterbaume_codepipeline::CodePipelineService::new());
    let cognitoidp =
        Arc::new(winterbaume_cognitoidentityprovider::CognitoIdentityProviderService::new());
    let config = Arc::new(winterbaume_config::ConfigService::new());
    let ecr = Arc::new(winterbaume_ecr::EcrService::new());
    let ecs = Arc::new(winterbaume_ecs::EcsService::new());
    let elasticache = Arc::new(winterbaume_elasticache::ElastiCacheService::new());
    let efs = Arc::new(winterbaume_efs::EfsService::new());
    let eks = Arc::new(winterbaume_eks::EksService::new());
    let elb = Arc::new(winterbaume_elasticloadbalancing::ElasticLoadBalancingService::new());
    let elbv2 = Arc::new(winterbaume_elasticloadbalancingv2::ElasticLoadBalancingV2Service::new());
    let emr_svc = Arc::new(winterbaume_emr::EmrService::new());
    let events = Arc::new(winterbaume_eventbridge::EventBridgeService::new());
    let firehose = Arc::new(winterbaume_firehose::FirehoseService::new());
    let fis = Arc::new(winterbaume_fis::FisService::new());
    let cloudcontrol = Arc::new(winterbaume_cloudcontrol::CloudControlService::new());
    let glue = Arc::new(winterbaume_glue::GlueService::new());
    let kafka = Arc::new(winterbaume_kafka::KafkaService::new());
    let kinesis = Arc::new(winterbaume_kinesis::KinesisService::new());
    let kms = Arc::new(winterbaume_kms::KmsService::new());
    let lambda = Arc::new(winterbaume_lambda::LambdaService::new());
    let logs = Arc::new(winterbaume_cloudwatchlogs::CloudWatchLogsService::new());
    let opensearch = Arc::new(winterbaume_opensearch::OpenSearchService::new());
    let opensearchserverless =
        Arc::new(winterbaume_opensearchserverless::OpenSearchServerlessService::new());
    let fsx_svc = Arc::new(winterbaume_fsx::FsxService::new());
    let pinpoint_svc = Arc::new(winterbaume_pinpoint::PinpointService::new());
    let redshift_svc = Arc::new(winterbaume_redshift::RedshiftService::new());
    let route53 = Arc::new(winterbaume_route53::Route53Service::new());
    let sagemaker_svc = Arc::new(winterbaume_sagemaker::SageMakerService::new());
    let servicecatalog_svc = Arc::new(winterbaume_servicecatalog::ServiceCatalogService::new());
    let scheduler = Arc::new(winterbaume_scheduler::SchedulerService::new());
    let secretsmanager = Arc::new(winterbaume_secretsmanager::SecretsManagerService::new());
    let ses = Arc::new(winterbaume_sesv2::SesV2Service::new());
    let sesv1 = Arc::new(winterbaume_ses::SesService::new());
    let sns = Arc::new(winterbaume_sns::SnsService::new());
    let ssm = Arc::new(winterbaume_ssm::SsmService::new());
    let ssoadmin = Arc::new(winterbaume_ssoadmin::SsoAdminService::new());
    let stepfunctions = Arc::new(winterbaume_sfn::SfnService::new());
    let wafv2 = Arc::new(winterbaume_wafv2::WafV2Service::new());
    let workspaces = Arc::new(winterbaume_workspaces::WorkSpacesService::new());

    let account = Arc::new(winterbaume_account::AccountService::new());
    let acmpca = Arc::new(winterbaume_acmpca::AcmPcaService::new());
    let amp = Arc::new(winterbaume_amp::AmpService::new());
    let appconfig = Arc::new(winterbaume_appconfig::AppConfigService::new());
    let applicationautoscaling =
        Arc::new(winterbaume_applicationautoscaling::ApplicationAutoScalingService::new());
    let appmesh = Arc::new(winterbaume_appmesh::AppMeshService::new());
    let autoscaling = Arc::new(winterbaume_autoscaling::AutoScalingService::new());
    let batch = Arc::new(winterbaume_batch::BatchService::new());
    let bedrock = Arc::new(winterbaume_bedrock::BedrockService::new());
    let bedrockagent = Arc::new(winterbaume_bedrockagent::BedrockAgentService::new());
    let budgets = Arc::new(winterbaume_budgets::BudgetsService::new());
    let cloudhsmv2 = Arc::new(winterbaume_cloudhsmv2::CloudHsmV2Service::new());
    let cloudtrail = Arc::new(winterbaume_cloudtrail::CloudTrailService::new());
    let cognitoidentity = Arc::new(winterbaume_cognitoidentity::CognitoIdentityService::new());
    let comprehend = Arc::new(winterbaume_comprehend::ComprehendService::new());
    let connect = Arc::new(winterbaume_connect::ConnectService::new());
    let costexplorer = Arc::new(winterbaume_costexplorer::CostExplorerService::new());
    let datapipeline = Arc::new(winterbaume_datapipeline::DataPipelineService::new());
    let datasync = Arc::new(winterbaume_datasync::DataSyncService::new());
    let directconnect = Arc::new(winterbaume_directconnect::DirectConnectService::new());
    let directory = Arc::new(winterbaume_directory::DirectoryService::new());
    let dms = Arc::new(winterbaume_databasemigration::DatabaseMigrationService::new());
    let dsql = Arc::new(winterbaume_dsql::DsqlService::new());
    let ec2 = Arc::new(winterbaume_ec2::Ec2Service::new());
    let emrcontainers = Arc::new(winterbaume_emrcontainers::EmrContainersService::new());
    let emrserverless = Arc::new(winterbaume_emrserverless::EmrServerlessService::new());
    let guardduty = Arc::new(winterbaume_guardduty::GuardDutyService::new());
    let identitystore = Arc::new(winterbaume_identitystore::IdentityStoreService::new());
    let inspector2 = Arc::new(winterbaume_inspector2::Inspector2Service::new());
    let iot = Arc::new(winterbaume_iot::IotService::new());
    let ivs = Arc::new(winterbaume_ivs::IvsService::new());
    let kinesisanalyticsv2 =
        Arc::new(winterbaume_kinesisanalyticsv2::KinesisAnalyticsV2Service::new());
    let kinesisvideo = Arc::new(winterbaume_kinesisvideo::KinesisVideoService::new());
    let lakeformation = Arc::new(winterbaume_lakeformation::LakeFormationService::new());
    let lexmodelsv2 = Arc::new(winterbaume_lexmodelsv2::LexModelsV2Service::new());
    let macie2 = Arc::new(winterbaume_macie2::Macie2Service::new());
    let medialive = Arc::new(winterbaume_medialive::MediaLiveService::new());
    let mediapackage = Arc::new(winterbaume_mediapackage::MediaPackageService::new());
    let mediapackagev2 = Arc::new(winterbaume_mediapackagev2::MediaPackageV2Service::new());
    let mediastore = Arc::new(winterbaume_mediastore::MediaStoreService::new());
    let mq = Arc::new(winterbaume_mq::MqService::new());
    let networkfirewall = Arc::new(winterbaume_networkfirewall::NetworkFirewallService::new());
    let networkmanager = Arc::new(winterbaume_networkmanager::NetworkManagerService::new());
    let organizations = Arc::new(winterbaume_organizations::OrganizationsService::new());
    let osis = Arc::new(winterbaume_osis::OsisService::new());
    let outposts = Arc::new(winterbaume_outposts::OutpostsService::new());
    let keyspaces = Arc::new(winterbaume_keyspaces::KeyspacesService::new());
    let rolesanywhere = Arc::new(winterbaume_rolesanywhere::RolesAnywhereService::new());
    let pipes = Arc::new(winterbaume_pipes::PipesService::new());
    let quicksight = Arc::new(winterbaume_quicksight::QuickSightService::new());
    let ram = Arc::new(winterbaume_ram::RamService::new());
    let rds = Arc::new(winterbaume_rds::RdsService::new());
    let rekognition = Arc::new(winterbaume_rekognition::RekognitionService::new());
    let resiliencehub = Arc::new(winterbaume_resiliencehub::ResilienceHubService::new());
    let resourcegroups = Arc::new(winterbaume_resourcegroups::ResourceGroupsService::new());
    let route53domains = Arc::new(winterbaume_route53domains::Route53DomainsService::new());
    let route53resolver = Arc::new(winterbaume_route53resolver::Route53ResolverService::new());
    let s3control = Arc::new(winterbaume_s3control::S3ControlService::new());
    let s3tables = Arc::new(winterbaume_s3tables::S3TablesService::new());
    let securityhub = Arc::new(winterbaume_securityhub::SecurityHubService::new());
    let servicecatalogappregistry =
        Arc::new(winterbaume_servicecatalogappregistry::ServiceCatalogAppRegistryService::new());
    let servicediscovery = Arc::new(winterbaume_servicediscovery::ServiceDiscoveryService::new());
    let servicequotas = Arc::new(winterbaume_servicequotas::ServiceQuotasService::new());
    let shield = Arc::new(winterbaume_shield::ShieldService::new());
    let signer = Arc::new(winterbaume_signer::SignerService::new());
    let simpledbv2 = Arc::new(winterbaume_simpledbv2::SimpleDbV2Service::new());
    let swf = Arc::new(winterbaume_swf::SwfService::new());
    let synthetics = Arc::new(winterbaume_synthetics::SyntheticsService::new());
    let timestreaminfluxdb =
        Arc::new(winterbaume_timestreaminfluxdb::TimestreamInfluxDbService::new());
    let timestreamquery = Arc::new(winterbaume_timestreamquery::TimestreamQueryService::new());
    let timestreamwrite = Arc::new(winterbaume_timestreamwrite::TimestreamWriteService::new());
    let transcribe = Arc::new(winterbaume_transcribe::TranscribeService::new());
    let transfer = Arc::new(winterbaume_transfer::TransferService::new());
    let vpclattice = Arc::new(winterbaume_vpclattice::VpcLatticeService::new());
    let xray = Arc::new(winterbaume_xray::XRayService::new());

    // --- Injectable references for Terraform state injection ----------------

    let injectable = InjectableServices {
        accessanalyzer: Arc::clone(&accessanalyzer),
        acm: Arc::clone(&acm),
        apigatewayv2: Arc::clone(&apigatewayv2),
        appfabric: Arc::clone(&appfabric),
        appflow: Arc::clone(&appflow),
        applicationcostprofiler: Arc::clone(&applicationcostprofiler),
        appsync: Arc::clone(&appsync),
        amplify: Arc::clone(&amplify),
        apprunner: Arc::clone(&apprunner),
        athena: Arc::clone(&athena),
        auditmanager: Arc::clone(&auditmanager),
        backup: Arc::clone(&backup),
        chatbot: Arc::clone(&chatbot),
        cloudformation: Arc::clone(&cloudformation),
        cloudfront: Arc::clone(&cloudfront),
        cloudwatch: Arc::clone(&cloudwatch),
        codeartifact: Arc::clone(&codeartifact),
        codebuild: Arc::clone(&codebuild),
        codecommit: Arc::clone(&codecommit),
        codedeploy: Arc::clone(&codedeploy),
        codepipeline: Arc::clone(&codepipeline),
        cognitoidp: Arc::clone(&cognitoidp),
        config: Arc::clone(&config),
        dynamodb: Arc::clone(&dynamodb),
        ecr: Arc::clone(&ecr),
        ecs: Arc::clone(&ecs),
        efs: Arc::clone(&efs),
        eks: Arc::clone(&eks),
        elasticache: Arc::clone(&elasticache),
        elbv2: Arc::clone(&elbv2),
        emr: Arc::clone(&emr_svc),
        events: Arc::clone(&events),
        firehose: Arc::clone(&firehose),
        fis: Arc::clone(&fis),
        glue: Arc::clone(&glue),
        iam: Arc::clone(&iam),
        kafka: Arc::clone(&kafka),
        kinesis: Arc::clone(&kinesis),
        kms: Arc::clone(&kms),
        lambda: Arc::clone(&lambda),
        logs: Arc::clone(&logs),
        opensearch: Arc::clone(&opensearch),
        opensearchserverless: Arc::clone(&opensearchserverless),
        redshift: Arc::clone(&redshift_svc),
        route53: Arc::clone(&route53),
        s3: Arc::clone(&s3),
        sagemaker: Arc::clone(&sagemaker_svc),
        scheduler: Arc::clone(&scheduler),
        secretsmanager: Arc::clone(&secretsmanager),
        ses: Arc::clone(&ses),
        sesv1: Arc::clone(&sesv1),
        sns: Arc::clone(&sns),
        sqs: Arc::clone(&sqs),
        ssm: Arc::clone(&ssm),
        ssoadmin: Arc::clone(&ssoadmin),
        stepfunctions: Arc::clone(&stepfunctions),
        wafv2: Arc::clone(&wafv2),
        workspaces: Arc::clone(&workspaces),
        account: Arc::clone(&account),
        acmpca: Arc::clone(&acmpca),
        amp: Arc::clone(&amp),
        apigateway: Arc::clone(&apigateway),
        appconfig: Arc::clone(&appconfig),
        applicationautoscaling: Arc::clone(&applicationautoscaling),
        appmesh: Arc::clone(&appmesh),
        autoscaling: Arc::clone(&autoscaling),
        batch: Arc::clone(&batch),
        bedrock: Arc::clone(&bedrock),
        bedrockagent: Arc::clone(&bedrockagent),
        budgets: Arc::clone(&budgets),
        cloudhsmv2: Arc::clone(&cloudhsmv2),
        cloudtrail: Arc::clone(&cloudtrail),
        cognitoidentity: Arc::clone(&cognitoidentity),
        comprehend: Arc::clone(&comprehend),
        connect: Arc::clone(&connect),
        costexplorer: Arc::clone(&costexplorer),
        datapipeline: Arc::clone(&datapipeline),
        datasync: Arc::clone(&datasync),
        directconnect: Arc::clone(&directconnect),
        directory: Arc::clone(&directory),
        dms: Arc::clone(&dms),
        dsql: Arc::clone(&dsql),
        ec2: Arc::clone(&ec2),
        elasticloadbalancing: Arc::clone(&elb),
        emrcontainers: Arc::clone(&emrcontainers),
        emrserverless: Arc::clone(&emrserverless),
        glacier: Arc::clone(&glacier),
        guardduty: Arc::clone(&guardduty),
        identitystore: Arc::clone(&identitystore),
        inspector2: Arc::clone(&inspector2),
        iot: Arc::clone(&iot),
        ivs: Arc::clone(&ivs),
        kinesisanalyticsv2: Arc::clone(&kinesisanalyticsv2),
        kinesisvideo: Arc::clone(&kinesisvideo),
        lakeformation: Arc::clone(&lakeformation),
        lexmodelsv2: Arc::clone(&lexmodelsv2),
        macie2: Arc::clone(&macie2),
        medialive: Arc::clone(&medialive),
        mediapackage: Arc::clone(&mediapackage),
        mediapackagev2: Arc::clone(&mediapackagev2),
        mediastore: Arc::clone(&mediastore),
        mq: Arc::clone(&mq),
        networkfirewall: Arc::clone(&networkfirewall),
        networkmanager: Arc::clone(&networkmanager),
        organizations: Arc::clone(&organizations),
        osis: Arc::clone(&osis),
        outposts: Arc::clone(&outposts),
        keyspaces: Arc::clone(&keyspaces),
        rolesanywhere: Arc::clone(&rolesanywhere),
        fsx: Arc::clone(&fsx_svc),
        pinpoint: Arc::clone(&pinpoint_svc),
        pipes: Arc::clone(&pipes),
        quicksight: Arc::clone(&quicksight),
        ram: Arc::clone(&ram),
        rds: Arc::clone(&rds),
        rekognition: Arc::clone(&rekognition),
        resiliencehub: Arc::clone(&resiliencehub),
        resourcegroups: Arc::clone(&resourcegroups),
        route53domains: Arc::clone(&route53domains),
        route53resolver: Arc::clone(&route53resolver),
        s3control: Arc::clone(&s3control),
        s3tables: Arc::clone(&s3tables),
        securityhub: Arc::clone(&securityhub),
        servicecatalog: Arc::clone(&servicecatalog_svc),
        servicecatalogappregistry: Arc::clone(&servicecatalogappregistry),
        servicediscovery: Arc::clone(&servicediscovery),
        servicequotas: Arc::clone(&servicequotas),
        shield: Arc::clone(&shield),
        signer: Arc::clone(&signer),
        simpledbv2: Arc::clone(&simpledbv2),
        swf: Arc::clone(&swf),
        synthetics: Arc::clone(&synthetics),
        timestreaminfluxdb: Arc::clone(&timestreaminfluxdb),
        timestreamquery: Arc::clone(&timestreamquery),
        timestreamwrite: Arc::clone(&timestreamwrite),
        transcribe: Arc::clone(&transcribe),
        transfer: Arc::clone(&transfer),
        vpclattice: Arc::clone(&vpclattice),
        xray: Arc::clone(&xray),
    };

    // --- Full service list --------------------------------------------------

    let mut services: Vec<Arc<dyn MockService>> = vec![
        Arc::new(winterbaume_sts::StsService::new()),
        iam,
        s3,
        sqs,
        dynamodb,
        kms,
        ssm,
        secretsmanager,
        logs,
        sns,
        lambda,
        ecr,
        events,
        stepfunctions,
        route53,
        cloudwatch,
        acm,
        eks,
        kinesis,
        cloudfront,
        firehose,
        fis,
        Arc::clone(&organizations) as Arc<dyn MockService>,
        ecs,
        elasticache,
        elb,
        elbv2,
        cognitoidp,
        ses,
        sesv1,
        Arc::new(winterbaume_ec2instanceconnect::Ec2InstanceConnectService::new()),
        Arc::clone(&ec2) as Arc<dyn MockService>,
        apigatewayv2,
        apigateway,
        apigatewaymanagementapi,
        Arc::clone(&account) as Arc<dyn MockService>,
        codecommit,
        Arc::clone(&rds) as Arc<dyn MockService>,
        Arc::new(winterbaume_rdsdata::RdsDataService::new()),
        Arc::new(winterbaume_neptune::NeptuneService::new()),
        Arc::new(winterbaume_polly::PollyService::new()),
        Arc::clone(&cloudformation) as Arc<dyn winterbaume_core::MockService>,
        Arc::clone(&cloudtrail) as Arc<dyn MockService>,
        Arc::clone(&budgets) as Arc<dyn MockService>,
        glacier,
        Arc::clone(&resourcegroups) as Arc<dyn MockService>,
        scheduler,
        Arc::clone(&appconfig) as Arc<dyn MockService>,
        codebuild,
        codepipeline,
        athena,
        auditmanager,
        chatbot,
        codeartifact,
        Arc::clone(&xray) as Arc<dyn MockService>,
        Arc::clone(&signer) as Arc<dyn MockService>,
        Arc::clone(&pipes) as Arc<dyn MockService>,
        Arc::new(winterbaume_dax::DaxService::new()),
        codedeploy,
        Arc::clone(&cognitoidentity) as Arc<dyn MockService>,
        Arc::clone(&guardduty) as Arc<dyn MockService>,
        backup,
        Arc::clone(&ivs) as Arc<dyn MockService>,
        wafv2,
        config,
        glue,
        Arc::clone(&fsx_svc) as Arc<dyn MockService>,
        Arc::new(winterbaume_memorydb::MemoryDbService::new()),
        ebs,
        Arc::clone(&networkfirewall) as Arc<dyn MockService>,
        Arc::clone(&sagemaker_svc) as Arc<dyn MockService>,
        appsync,
        Arc::clone(&iot) as Arc<dyn MockService>,
        kafka,
        Arc::clone(&lexmodelsv2) as Arc<dyn MockService>,
        Arc::clone(&lakeformation) as Arc<dyn MockService>,
        Arc::clone(&amp) as Arc<dyn MockService>,
        opensearch,
        Arc::clone(&opensearchserverless) as Arc<dyn MockService>,
        Arc::clone(&costexplorer) as Arc<dyn MockService>,
        Arc::clone(&bedrock) as Arc<dyn MockService>,
        Arc::new(winterbaume_clouddirectory::CloudDirectoryService::new()),
        Arc::clone(&cloudhsmv2) as Arc<dyn MockService>,
        Arc::new(winterbaume_dynamodbstreams::DynamoDbStreamsService::new()),
        Arc::clone(&dsql) as Arc<dyn MockService>,
        Arc::new(winterbaume_databrew::DataBrewService::new()),
        emr_svc,
        Arc::clone(&emrserverless) as Arc<dyn MockService>,
        Arc::clone(&workspaces) as Arc<dyn MockService>,
        Arc::clone(&emrcontainers) as Arc<dyn MockService>,
        Arc::clone(&bedrockagent) as Arc<dyn MockService>,
        Arc::clone(&connect) as Arc<dyn MockService>,
        Arc::clone(&kinesisanalyticsv2) as Arc<dyn MockService>,
        Arc::clone(&appmesh) as Arc<dyn MockService>,
        Arc::new(winterbaume_greengrass::GreengrassService::new()),
        Arc::clone(&networkmanager) as Arc<dyn MockService>,
        Arc::clone(&pinpoint_svc) as Arc<dyn MockService>,
        Arc::clone(&swf) as Arc<dyn MockService>,
        Arc::clone(&redshift_svc) as Arc<dyn MockService>,
        Arc::new({
            #[cfg(feature = "backend-sqlengine-duckdb")]
            {
                match &duckdb_conn {
                    Some(conn) => {
                        winterbaume_redshiftdata::RedshiftDataService::with_query_backend(Arc::new(
                            winterbaume_sqlengine_duckdb::DuckDbRedshiftQueryBackend::new(
                                Arc::clone(conn),
                            ),
                        ))
                    }
                    None => winterbaume_redshiftdata::RedshiftDataService::new(),
                }
            }
            #[cfg(not(feature = "backend-sqlengine-duckdb"))]
            {
                winterbaume_redshiftdata::RedshiftDataService::new()
            }
        }),
        Arc::new(winterbaume_mediaconnect::MediaConnectService::new()),
        Arc::clone(&medialive) as Arc<dyn MockService>,
        Arc::clone(&mediapackage) as Arc<dyn MockService>,
        Arc::new(winterbaume_managedblockchain::ManagedBlockchainService::new()),
        Arc::clone(&resiliencehub) as Arc<dyn MockService>,
        Arc::clone(&quicksight) as Arc<dyn MockService>,
        Arc::clone(&vpclattice) as Arc<dyn MockService>,
        Arc::new(winterbaume_workspacesweb::WorkspacesWebService::new()),
        Arc::clone(&mediastore) as Arc<dyn MockService>,
        Arc::clone(&route53domains) as Arc<dyn MockService>,
        Arc::clone(&mediapackagev2) as Arc<dyn MockService>,
        Arc::clone(&osis) as Arc<dyn MockService>,
        Arc::clone(&outposts) as Arc<dyn MockService>,
        Arc::new(winterbaume_cloudcontrol::CloudControlService::new()),
        Arc::clone(&keyspaces) as Arc<dyn MockService>,
        Arc::clone(&rolesanywhere) as Arc<dyn MockService>,
        Arc::clone(&servicecatalog_svc) as Arc<dyn MockService>,
        Arc::clone(&timestreamwrite) as Arc<dyn MockService>,
        Arc::new(winterbaume_connectcampaigns::ConnectCampaignsService::new()),
        Arc::clone(&securityhub) as Arc<dyn MockService>,
        Arc::clone(&inspector2) as Arc<dyn MockService>,
        Arc::clone(&rekognition) as Arc<dyn MockService>,
        Arc::clone(&synthetics) as Arc<dyn MockService>,
        Arc::clone(&shield) as Arc<dyn MockService>,
        Arc::clone(&servicequotas) as Arc<dyn MockService>,
        Arc::clone(&ram) as Arc<dyn MockService>,
        Arc::clone(&macie2) as Arc<dyn MockService>,
        Arc::clone(&comprehend) as Arc<dyn MockService>,
        Arc::clone(&transcribe) as Arc<dyn MockService>,
        Arc::new(winterbaume_textract::TextractService::new()),
        Arc::new(winterbaume_forecast::ForecastService::new()),
        Arc::new(winterbaume_personalize::PersonalizeService::new()),
        Arc::clone(&route53resolver) as Arc<dyn MockService>,
        Arc::clone(&acmpca) as Arc<dyn MockService>,
        Arc::clone(&directconnect) as Arc<dyn MockService>,
        Arc::clone(&dms) as Arc<dyn MockService>,
        Arc::clone(&directory) as Arc<dyn MockService>,
        Arc::clone(&mq) as Arc<dyn MockService>,
        Arc::clone(&transfer) as Arc<dyn MockService>,
        Arc::clone(&datasync) as Arc<dyn MockService>,
        Arc::clone(&servicediscovery) as Arc<dyn MockService>,
        Arc::clone(&identitystore) as Arc<dyn MockService>,
        efs,
        Arc::new(winterbaume_support::SupportService::new()),
        Arc::clone(&batch) as Arc<dyn MockService>,
        Arc::clone(&kinesisvideo) as Arc<dyn MockService>,
        Arc::new(winterbaume_sso::SsoService::new()),
        Arc::clone(&ssoadmin) as Arc<dyn MockService>,
        Arc::clone(&servicecatalogappregistry) as Arc<dyn MockService>,
        Arc::clone(&timestreamquery) as Arc<dyn MockService>,
        Arc::clone(&timestreaminfluxdb) as Arc<dyn MockService>,
        Arc::new(winterbaume_marketplacemetering::MarketplaceMeteringService::new()),
        Arc::new(winterbaume_sagemakermetrics::SageMakerMetricsService::new()),
        Arc::new(winterbaume_sagemakerruntime::SageMakerRuntimeService::new()),
        Arc::new(winterbaume_iotdataplane::IotDataPlaneService::new()),
        Arc::new(winterbaume_kinesisvideoarchivedmedia::KinesisVideoArchivedMediaService::new()),
        Arc::new(winterbaume_resourcegroupstagging::TaggingService::new()),
        Arc::clone(&applicationautoscaling) as Arc<dyn MockService>,
        Arc::clone(&autoscaling) as Arc<dyn MockService>,
        Arc::new(winterbaume_mediastoredata::MediaStoreDataService::new()),
        Arc::clone(&simpledbv2) as Arc<dyn MockService>,
        Arc::clone(&datapipeline) as Arc<dyn MockService>,
        Arc::clone(&s3control) as Arc<dyn MockService>,
        Arc::new(winterbaume_s3files::S3FilesService::new()),
        Arc::new(winterbaume_s3vectors::S3VectorsService::new()),
        Arc::clone(&s3tables) as Arc<dyn MockService>,
        Arc::new(winterbaume_panorama::PanoramaService::new()),
        Arc::new(winterbaume_elasticbeanstalk::ElasticBeanstalkService::new()),
        Arc::clone(&apprunner) as Arc<dyn winterbaume_core::MockService>,
        Arc::clone(&amplify) as Arc<dyn winterbaume_core::MockService>,
        Arc::clone(&accessanalyzer) as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_aiops::AIOpsService::new()) as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_amplifybackend::AmplifyBackendService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_appconfigdata::AppConfigDataService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::clone(&appfabric) as Arc<dyn winterbaume_core::MockService>,
        Arc::clone(&appflow) as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_appintegrations::AppIntegrationsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::clone(&applicationcostprofiler) as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_applicationdiscoveryservice::ApplicationDiscoveryService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_applicationsignals::ApplicationSignalsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_arczonalshift::ArcZonalShiftService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_artifact::ArtifactService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_autoscalingplans::AutoScalingPlansService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_backupgateway::BackupGatewayService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_backupsearch::BackupSearchService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_bcmdashboards::BcmDashboardsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_bcmdataexports::BcmDataExportsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_bcmrecommendedactions::BcmRecommendedActionsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_dlm::DlmService::new()) as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_rbin::RbinService::new()) as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_savingsplans::SavingsPlansService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_personalizeevents::PersonalizeEventsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_personalizeruntime::PersonalizeRuntimeService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_pinpointsmsvoice::PinpointSmsVoiceService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_pi::PiService::new()) as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_trustedadvisor::TrustedAdvisorService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_supportapp::SupportAppService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_pricing::PricingService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_freetier::FreeTierService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_cloud9::Cloud9Service::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_s3outposts::S3OutpostsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_route53recoverycluster::RecoveryClusterService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_snowdevicemanagement::SnowDeviceManagementService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_simspaceweaver::SimSpaceWeaverService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_taxsettings::TaxSettingsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_ssmquicksetup::SsmQuickSetupService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_pcaconnectorscep::PcaConnectorScepService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_cloudfrontkeyvaluestore::CloudFrontKvsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_cloudtraildata::CloudTrailDataService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_cloudsearchdomain::CloudSearchDomainService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_connectparticipant::ConnectParticipantService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_controlcatalog::ControlCatalogService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_costandusagereport::CostAndUsageReportService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_braket::BraketService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_chimesdkmeetings::ChimeSdkMeetingsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_cognitosync::CognitoSyncService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_codestarnotifications::CodeStarNotificationsService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_codegurusecurity::CodeGuruSecurityService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_codegurureviewer::CodeGuruReviewerService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_connectcontactlens::ConnectContactLensService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_costoptimizationhub::CostOptimizationHubService::new())
            as Arc<dyn winterbaume_core::MockService>,
        Arc::new(winterbaume_billing::BillingService::new())
            as Arc<dyn winterbaume_core::MockService>,
    ];

    Ok((services, injectable))
}

// ---------------------------------------------------------------------------
// Terraform state injection
// ---------------------------------------------------------------------------

/// Load a terraform state file and inject supported resources into service backends.
async fn load_tfstate(
    path: &str,
    injectable: &InjectableServices,
    account_id: &str,
    default_region: &str,
) -> Result<winterbaume_terraform::InjectionReport, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let tfstate = winterbaume_tfstate::TerraformState::from_reader(file)?;

    let mut injector = winterbaume_terraform::TerraformInjector::new();

    // Register all available converters
    use winterbaume_terraform::converters::{
        accessanalyzer, account, acm, acmpca, amp, amplify, apigateway, apigatewayv2, appconfig,
        appfabric, appflow, applicationautoscaling, applicationcostprofiler, appmesh, apprunner,
        appsync, athena, auditmanager, autoscaling, backup, batch, bedrock, bedrockagent, budgets,
        chatbot, cloudformation, cloudfront, cloudhsmv2, cloudtrail, cloudwatch, codeartifact,
        codebuild, codecommit, codedeploy, codepipeline, cognitoidentity, cognitoidp, comprehend,
        config, connect, costexplorer, datapipeline, datasync, directconnect, directory, dms, dsql,
        dynamodb, ec2, ecr, ecs, efs, eks, elasticache, elasticloadbalancing, elbv2, emr,
        emrcontainers, emrserverless, events, firehose, fis, fsx, glacier, glue, guardduty, iam,
        identitystore, inspector2, iot, ivs, kafka, keyspaces, kinesis, kinesisanalyticsv2,
        kinesisvideo, kms, lakeformation, lambda, lexmodelsv2, logs, macie2, medialive,
        mediapackage, mediapackagev2, mediastore, mq, networkfirewall, networkmanager, opensearch,
        opensearchserverless, organizations, osis, outposts, pinpoint, pipes, quicksight, ram, rds,
        redshift, rekognition, resiliencehub, resourcegroups, rolesanywhere, route53,
        route53domains, route53resolver, s3, s3control, s3tables, sagemaker, scheduler,
        secretsmanager, securityhub, servicecatalog, servicecatalogappregistry, servicediscovery,
        servicequotas, ses, sesv1, shield, signer, simpledbv2, sns, sqs, ssm, ssoadmin,
        stepfunctions, swf, synthetics, timestreaminfluxdb, timestreamquery, timestreamwrite,
        transcribe, transfer, vpclattice, wafv2, workspaces, xray,
    };
    injector.register(accessanalyzer::AwsAccessAnalyzerAnalyzerConverter::new(
        Arc::clone(&injectable.accessanalyzer),
    ));
    injector.register(appfabric::AwsAppFabricAppBundleConverter::new(Arc::clone(
        &injectable.appfabric,
    )));
    injector.register(appflow::AwsAppFlowFlowConverter::new(Arc::clone(
        &injectable.appflow,
    )));
    injector.register(
        applicationcostprofiler::AwsApplicationCostProfilerReportDefinitionConverter::new(
            Arc::clone(&injectable.applicationcostprofiler),
        ),
    );
    injector.register(account::AwsAccountAlternateContactConverter::new(
        Arc::clone(&injectable.account),
    ));
    injector.register(acm::AwsAcmCertificateConverter::new(Arc::clone(
        &injectable.acm,
    )));
    injector.register(acmpca::AwsAcmpcaCertificateAuthorityConverter::new(
        Arc::clone(&injectable.acmpca),
    ));
    injector.register(amp::AwsPrometheusWorkspaceConverter::new(Arc::clone(
        &injectable.amp,
    )));
    injector.register(amplify::AwsAmplifyAppConverter::new(Arc::clone(
        &injectable.amplify,
    )));
    injector.register(amplify::AwsAmplifyBranchConverter::new(Arc::clone(
        &injectable.amplify,
    )));
    injector.register(apigateway::AwsApiGatewayRestApiConverter::new(Arc::clone(
        &injectable.apigateway,
    )));
    injector.register(apigateway::AwsApiGatewayResourceConverter::new(Arc::clone(
        &injectable.apigateway,
    )));
    injector.register(apigateway::AwsApiGatewayMethodConverter::new(Arc::clone(
        &injectable.apigateway,
    )));
    injector.register(apigateway::AwsApiGatewayStageConverter::new(Arc::clone(
        &injectable.apigateway,
    )));
    injector.register(apigateway::AwsApiGatewayDeploymentConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayApiKeyConverter::new(Arc::clone(
        &injectable.apigateway,
    )));
    injector.register(apigateway::AwsApiGatewayAccountConverter::new(Arc::clone(
        &injectable.apigateway,
    )));
    injector.register(apigateway::AwsApiGatewayAuthorizerConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayBasePathMappingConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayClientCertificateConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayDocumentationPartConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayDocumentationVersionConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayDomainNameConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(
        apigateway::AwsApiGatewayDomainNameAccessAssociationConverter::new(Arc::clone(
            &injectable.apigateway,
        )),
    );
    injector.register(apigateway::AwsApiGatewayGatewayResponseConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayIntegrationConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayIntegrationResponseConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayMethodResponseConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayModelConverter::new(Arc::clone(
        &injectable.apigateway,
    )));
    injector.register(apigateway::AwsApiGatewayRequestValidatorConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayRestApiPolicyConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayUsagePlanConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayUsagePlanKeyConverter::new(
        Arc::clone(&injectable.apigateway),
    ));
    injector.register(apigateway::AwsApiGatewayVpcLinkConverter::new(Arc::clone(
        &injectable.apigateway,
    )));
    injector.register(apigatewayv2::AwsApigatewayv2ApiConverter::new(Arc::clone(
        &injectable.apigatewayv2,
    )));
    injector.register(apigatewayv2::AwsApigatewayv2ApiMappingConverter::new(
        Arc::clone(&injectable.apigatewayv2),
    ));
    injector.register(apigatewayv2::AwsApigatewayv2AuthorizerConverter::new(
        Arc::clone(&injectable.apigatewayv2),
    ));
    injector.register(apigatewayv2::AwsApigatewayv2DeploymentConverter::new(
        Arc::clone(&injectable.apigatewayv2),
    ));
    injector.register(apigatewayv2::AwsApigatewayv2DomainNameConverter::new(
        Arc::clone(&injectable.apigatewayv2),
    ));
    injector.register(apigatewayv2::AwsApigatewayv2IntegrationConverter::new(
        Arc::clone(&injectable.apigatewayv2),
    ));
    injector.register(
        apigatewayv2::AwsApigatewayv2IntegrationResponseConverter::new(Arc::clone(
            &injectable.apigatewayv2,
        )),
    );
    injector.register(apigatewayv2::AwsApigatewayv2ModelConverter::new(
        Arc::clone(&injectable.apigatewayv2),
    ));
    injector.register(apigatewayv2::AwsApigatewayv2RouteConverter::new(
        Arc::clone(&injectable.apigatewayv2),
    ));
    injector.register(apigatewayv2::AwsApigatewayv2RouteResponseConverter::new(
        Arc::clone(&injectable.apigatewayv2),
    ));
    injector.register(apigatewayv2::AwsApigatewayv2StageConverter::new(
        Arc::clone(&injectable.apigatewayv2),
    ));
    injector.register(apigatewayv2::AwsApigatewayv2VpcLinkConverter::new(
        Arc::clone(&injectable.apigatewayv2),
    ));
    injector.register(appconfig::AwsAppconfigApplicationConverter::new(
        Arc::clone(&injectable.appconfig),
    ));
    injector.register(appconfig::AwsAppconfigEnvironmentConverter::new(
        Arc::clone(&injectable.appconfig),
    ));
    injector.register(appconfig::AwsAppconfigConfigurationProfileConverter::new(
        Arc::clone(&injectable.appconfig),
    ));
    injector.register(appconfig::AwsAppconfigDeploymentStrategyConverter::new(
        Arc::clone(&injectable.appconfig),
    ));
    injector.register(
        applicationautoscaling::AwsAppautoscalingTargetConverter::new(Arc::clone(
            &injectable.applicationautoscaling,
        )),
    );
    injector.register(
        applicationautoscaling::AwsAppautoscalingPolicyConverter::new(Arc::clone(
            &injectable.applicationautoscaling,
        )),
    );
    injector.register(appmesh::AwsAppmeshMeshConverter::new(Arc::clone(
        &injectable.appmesh,
    )));
    injector.register(appmesh::AwsAppmeshVirtualNodeConverter::new(Arc::clone(
        &injectable.appmesh,
    )));
    injector.register(appmesh::AwsAppmeshVirtualServiceConverter::new(Arc::clone(
        &injectable.appmesh,
    )));
    injector.register(apprunner::AwsAppRunnerServiceConverter::new(Arc::clone(
        &injectable.apprunner,
    )));
    injector.register(
        apprunner::AwsAppRunnerAutoScalingConfigurationVersionConverter::new(Arc::clone(
            &injectable.apprunner,
        )),
    );
    injector.register(apprunner::AwsAppRunnerConnectionConverter::new(Arc::clone(
        &injectable.apprunner,
    )));
    injector.register(
        apprunner::AwsAppRunnerCustomDomainAssociationConverter::new(Arc::clone(
            &injectable.apprunner,
        )),
    );
    injector.register(
        apprunner::AwsAppRunnerDefaultAutoScalingConfigurationVersionConverter::new(Arc::clone(
            &injectable.apprunner,
        )),
    );
    injector.register(apprunner::AwsAppRunnerDeploymentConverter::new(Arc::clone(
        &injectable.apprunner,
    )));
    injector.register(
        apprunner::AwsAppRunnerObservabilityConfigurationConverter::new(Arc::clone(
            &injectable.apprunner,
        )),
    );
    injector.register(apprunner::AwsAppRunnerVpcConnectorConverter::new(
        Arc::clone(&injectable.apprunner),
    ));
    injector.register(apprunner::AwsAppRunnerVpcIngressConnectionConverter::new(
        Arc::clone(&injectable.apprunner),
    ));
    injector.register(appsync::AwsAppsyncGraphqlApiConverter::new(Arc::clone(
        &injectable.appsync,
    )));
    injector.register(appsync::AwsAppsyncApiCacheConverter::new(Arc::clone(
        &injectable.appsync,
    )));
    injector.register(appsync::AwsAppsyncApiKeyConverter::new(Arc::clone(
        &injectable.appsync,
    )));
    injector.register(appsync::AwsAppsyncDatasourceConverter::new(Arc::clone(
        &injectable.appsync,
    )));
    injector.register(appsync::AwsAppsyncDomainNameConverter::new(Arc::clone(
        &injectable.appsync,
    )));
    injector.register(appsync::AwsAppsyncDomainNameApiAssociationConverter::new(
        Arc::clone(&injectable.appsync),
    ));
    injector.register(appsync::AwsAppsyncFunctionConverter::new(Arc::clone(
        &injectable.appsync,
    )));
    injector.register(appsync::AwsAppsyncResolverConverter::new(Arc::clone(
        &injectable.appsync,
    )));
    injector.register(appsync::AwsAppsyncSourceApiAssociationConverter::new(
        Arc::clone(&injectable.appsync),
    ));
    injector.register(appsync::AwsAppsyncTypeConverter::new(Arc::clone(
        &injectable.appsync,
    )));
    injector.register(athena::AwsAthenaWorkgroupConverter::new(Arc::clone(
        &injectable.athena,
    )));
    injector.register(athena::AwsAthenaDataCatalogConverter::new(Arc::clone(
        &injectable.athena,
    )));
    injector.register(auditmanager::AwsAuditManagerControlConverter::new(
        Arc::clone(&injectable.auditmanager),
    ));
    injector.register(auditmanager::AwsAuditManagerFrameworkConverter::new(
        Arc::clone(&injectable.auditmanager),
    ));
    injector.register(
        auditmanager::AwsAuditManagerAccountRegistrationConverter::new(Arc::clone(
            &injectable.auditmanager,
        )),
    );
    injector.register(auditmanager::AwsAuditManagerAssessmentConverter::new(
        Arc::clone(&injectable.auditmanager),
    ));
    injector.register(
        auditmanager::AwsAuditManagerAssessmentDelegationConverter::new(Arc::clone(
            &injectable.auditmanager,
        )),
    );
    injector.register(auditmanager::AwsAuditManagerAssessmentReportConverter::new(
        Arc::clone(&injectable.auditmanager),
    ));
    injector.register(auditmanager::AwsAuditManagerFrameworkShareConverter::new(
        Arc::clone(&injectable.auditmanager),
    ));
    injector.register(
        auditmanager::AwsAuditManagerOrganizationAdminAccountRegistrationConverter::new(
            Arc::clone(&injectable.auditmanager),
        ),
    );
    injector.register(autoscaling::AwsAutoscalingGroupConverter::new(Arc::clone(
        &injectable.autoscaling,
    )));
    injector.register(autoscaling::AwsLaunchConfigurationConverter::new(
        Arc::clone(&injectable.autoscaling),
    ));
    injector.register(autoscaling::AwsAutoscalingPolicyConverter::new(Arc::clone(
        &injectable.autoscaling,
    )));
    injector.register(autoscaling::AwsAutoscalingScheduleConverter::new(
        Arc::clone(&injectable.autoscaling),
    ));
    injector.register(autoscaling::AwsAutoscalingAttachmentConverter::new(
        Arc::clone(&injectable.autoscaling),
    ));
    injector.register(autoscaling::AwsAutoscalingGroupTagConverter::new(
        Arc::clone(&injectable.autoscaling),
    ));
    injector.register(autoscaling::AwsAutoscalingLifecycleHookConverter::new(
        Arc::clone(&injectable.autoscaling),
    ));
    injector.register(autoscaling::AwsAutoscalingNotificationConverter::new(
        Arc::clone(&injectable.autoscaling),
    ));
    injector.register(
        autoscaling::AwsAutoscalingTrafficSourceAttachmentConverter::new(Arc::clone(
            &injectable.autoscaling,
        )),
    );
    injector.register(backup::AwsBackupVaultConverter::new(Arc::clone(
        &injectable.backup,
    )));
    injector.register(backup::AwsBackupPlanConverter::new(Arc::clone(
        &injectable.backup,
    )));
    injector.register(backup::AwsBackupFrameworkConverter::new(Arc::clone(
        &injectable.backup,
    )));
    injector.register(backup::AwsBackupGlobalSettingsConverter::new(Arc::clone(
        &injectable.backup,
    )));
    injector.register(backup::AwsBackupLogicallyAirGappedVaultConverter::new(
        Arc::clone(&injectable.backup),
    ));
    injector.register(backup::AwsBackupRegionSettingsConverter::new(Arc::clone(
        &injectable.backup,
    )));
    injector.register(backup::AwsBackupReportPlanConverter::new(Arc::clone(
        &injectable.backup,
    )));
    injector.register(backup::AwsBackupRestoreTestingPlanConverter::new(
        Arc::clone(&injectable.backup),
    ));
    injector.register(backup::AwsBackupRestoreTestingSelectionConverter::new(
        Arc::clone(&injectable.backup),
    ));
    injector.register(backup::AwsBackupSelectionConverter::new(Arc::clone(
        &injectable.backup,
    )));
    injector.register(backup::AwsBackupVaultLockConfigurationConverter::new(
        Arc::clone(&injectable.backup),
    ));
    injector.register(backup::AwsBackupVaultNotificationsConverter::new(
        Arc::clone(&injectable.backup),
    ));
    injector.register(backup::AwsBackupVaultPolicyConverter::new(Arc::clone(
        &injectable.backup,
    )));
    injector.register(batch::AwsBatchComputeEnvironmentConverter::new(Arc::clone(
        &injectable.batch,
    )));
    injector.register(batch::AwsBatchJobQueueConverter::new(Arc::clone(
        &injectable.batch,
    )));
    injector.register(batch::AwsBatchJobDefinitionConverter::new(Arc::clone(
        &injectable.batch,
    )));
    injector.register(batch::AwsBatchSchedulingPolicyConverter::new(Arc::clone(
        &injectable.batch,
    )));
    injector.register(bedrock::AwsBedrockGuardrailConverter::new(Arc::clone(
        &injectable.bedrock,
    )));
    injector.register(
        bedrock::AwsBedrockModelInvocationLoggingConfigurationConverter::new(Arc::clone(
            &injectable.bedrock,
        )),
    );
    injector.register(bedrockagent::AwsBedrockagentAgentConverter::new(
        Arc::clone(&injectable.bedrockagent),
    ));
    injector.register(bedrockagent::AwsBedrockagentKnowledgeBaseConverter::new(
        Arc::clone(&injectable.bedrockagent),
    ));
    injector.register(bedrockagent::AwsBedrockagentAgentActionGroupConverter::new(
        Arc::clone(&injectable.bedrockagent),
    ));
    injector.register(bedrockagent::AwsBedrockagentAgentAliasConverter::new(
        Arc::clone(&injectable.bedrockagent),
    ));
    injector.register(
        bedrockagent::AwsBedrockagentAgentCollaboratorConverter::new(Arc::clone(
            &injectable.bedrockagent,
        )),
    );
    injector.register(
        bedrockagent::AwsBedrockagentAgentKnowledgeBaseAssociationConverter::new(Arc::clone(
            &injectable.bedrockagent,
        )),
    );
    injector.register(bedrockagent::AwsBedrockagentDataSourceConverter::new(
        Arc::clone(&injectable.bedrockagent),
    ));
    injector.register(bedrockagent::AwsBedrockagentPromptConverter::new(
        Arc::clone(&injectable.bedrockagent),
    ));
    injector.register(budgets::AwsBudgetsBudgetConverter::new(Arc::clone(
        &injectable.budgets,
    )));
    injector.register(chatbot::AwsChatbotSlackChannelConfigurationConverter::new(
        Arc::clone(&injectable.chatbot),
    ));
    injector.register(chatbot::AwsChatbotTeamsChannelConfigurationConverter::new(
        Arc::clone(&injectable.chatbot),
    ));
    injector.register(cloudformation::AwsCloudformationStackConverter::new(
        Arc::clone(&injectable.cloudformation),
    ));
    injector.register(cloudfront::AwsCloudfrontDistributionConverter::new(
        Arc::clone(&injectable.cloudfront),
    ));
    injector.register(cloudfront::AwsCloudfrontCachePolicyConverter::new(
        Arc::clone(&injectable.cloudfront),
    ));
    injector.register(
        cloudfront::AwsCloudfrontContinuousDeploymentPolicyConverter::new(Arc::clone(
            &injectable.cloudfront,
        )),
    );
    injector.register(
        cloudfront::AwsCloudfrontFieldLevelEncryptionConfigConverter::new(Arc::clone(
            &injectable.cloudfront,
        )),
    );
    injector.register(
        cloudfront::AwsCloudfrontFieldLevelEncryptionProfileConverter::new(Arc::clone(
            &injectable.cloudfront,
        )),
    );
    injector.register(cloudfront::AwsCloudfrontFunctionConverter::new(Arc::clone(
        &injectable.cloudfront,
    )));
    injector.register(cloudfront::AwsCloudfrontKeyGroupConverter::new(Arc::clone(
        &injectable.cloudfront,
    )));
    injector.register(cloudfront::AwsCloudfrontKeyValueStoreConverter::new(
        Arc::clone(&injectable.cloudfront),
    ));
    injector.register(
        cloudfront::AwsCloudfrontMonitoringSubscriptionConverter::new(Arc::clone(
            &injectable.cloudfront,
        )),
    );
    injector.register(cloudfront::AwsCloudfrontOriginAccessControlConverter::new(
        Arc::clone(&injectable.cloudfront),
    ));
    injector.register(cloudfront::AwsCloudfrontOriginAccessIdentityConverter::new(
        Arc::clone(&injectable.cloudfront),
    ));
    injector.register(cloudfront::AwsCloudfrontOriginRequestPolicyConverter::new(
        Arc::clone(&injectable.cloudfront),
    ));
    injector.register(cloudfront::AwsCloudfrontPublicKeyConverter::new(
        Arc::clone(&injectable.cloudfront),
    ));
    injector.register(cloudfront::AwsCloudfrontRealtimeLogConfigConverter::new(
        Arc::clone(&injectable.cloudfront),
    ));
    injector.register(
        cloudfront::AwsCloudfrontResponseHeadersPolicyConverter::new(Arc::clone(
            &injectable.cloudfront,
        )),
    );
    injector.register(cloudfront::AwsCloudfrontVpcOriginConverter::new(
        Arc::clone(&injectable.cloudfront),
    ));
    injector.register(cloudhsmv2::AwsCloudHsmV2ClusterConverter::new(Arc::clone(
        &injectable.cloudhsmv2,
    )));
    injector.register(cloudtrail::AwsCloudtrailConverter::new(Arc::clone(
        &injectable.cloudtrail,
    )));
    injector.register(cloudwatch::AwsCloudwatchMetricAlarmConverter::new(
        Arc::clone(&injectable.cloudwatch),
    ));
    injector.register(codeartifact::AwsCodeArtifactDomainConverter::new(
        Arc::clone(&injectable.codeartifact),
    ));
    injector.register(codeartifact::AwsCodeArtifactRepositoryConverter::new(
        Arc::clone(&injectable.codeartifact),
    ));
    injector.register(codebuild::AwsCodebuildProjectConverter::new(Arc::clone(
        &injectable.codebuild,
    )));
    injector.register(codecommit::AwsCodecommitRepositoryConverter::new(
        Arc::clone(&injectable.codecommit),
    ));
    injector.register(codedeploy::AwsCodedeployAppConverter::new(Arc::clone(
        &injectable.codedeploy,
    )));
    injector.register(codedeploy::AwsCodedeployDeploymentGroupConverter::new(
        Arc::clone(&injectable.codedeploy),
    ));
    injector.register(codepipeline::AwsCodepipelinePipelineConverter::new(
        Arc::clone(&injectable.codepipeline),
    ));
    injector.register(cognitoidentity::AwsCognitoIdentityPoolConverter::new(
        Arc::clone(&injectable.cognitoidentity),
    ));
    injector.register(cognitoidp::AwsCognitoUserPoolConverter::new(Arc::clone(
        &injectable.cognitoidp,
    )));
    injector.register(cognitoidp::AwsCognitoUserPoolClientConverter::new(
        Arc::clone(&injectable.cognitoidp),
    ));
    injector.register(cognitoidp::AwsCognitoIdentityProviderConverter::new(
        Arc::clone(&injectable.cognitoidp),
    ));
    injector.register(cognitoidp::AwsCognitoManagedUserPoolClientConverter::new(
        Arc::clone(&injectable.cognitoidp),
    ));
    injector.register(cognitoidp::AwsCognitoResourceServerConverter::new(
        Arc::clone(&injectable.cognitoidp),
    ));
    injector.register(cognitoidp::AwsCognitoUserGroupConverter::new(Arc::clone(
        &injectable.cognitoidp,
    )));
    injector.register(cognitoidp::AwsCognitoUserInGroupConverter::new(Arc::clone(
        &injectable.cognitoidp,
    )));
    injector.register(cognitoidp::AwsCognitoUserPoolDomainConverter::new(
        Arc::clone(&injectable.cognitoidp),
    ));
    injector.register(cognitoidp::AwsCognitoUserPoolUiCustomizationConverter::new(
        Arc::clone(&injectable.cognitoidp),
    ));
    injector.register(comprehend::AwsComprehendEntityRecognizerConverter::new(
        Arc::clone(&injectable.comprehend),
    ));
    injector.register(config::AwsConfigConfigurationRecorderConverter::new(
        Arc::clone(&injectable.config),
    ));
    injector.register(config::AwsConfigConfigRuleConverter::new(Arc::clone(
        &injectable.config,
    )));
    injector.register(config::AwsConfigDeliveryChannelConverter::new(Arc::clone(
        &injectable.config,
    )));
    injector.register(config::AwsConfigAggregateAuthorizationConverter::new(
        Arc::clone(&injectable.config),
    ));
    injector.register(config::AwsConfigConfigurationAggregatorConverter::new(
        Arc::clone(&injectable.config),
    ));
    injector.register(config::AwsConfigConfigurationRecorderStatusConverter::new(
        Arc::clone(&injectable.config),
    ));
    injector.register(config::AwsConfigConformancePackConverter::new(Arc::clone(
        &injectable.config,
    )));
    injector.register(config::AwsConfigOrganizationConformancePackConverter::new(
        Arc::clone(&injectable.config),
    ));
    injector.register(config::AwsConfigOrganizationCustomPolicyRuleConverter::new(
        Arc::clone(&injectable.config),
    ));
    injector.register(config::AwsConfigOrganizationCustomRuleConverter::new(
        Arc::clone(&injectable.config),
    ));
    injector.register(config::AwsConfigOrganizationManagedRuleConverter::new(
        Arc::clone(&injectable.config),
    ));
    injector.register(config::AwsConfigRemediationConfigurationConverter::new(
        Arc::clone(&injectable.config),
    ));
    injector.register(config::AwsConfigRetentionConfigurationConverter::new(
        Arc::clone(&injectable.config),
    ));
    injector.register(connect::AwsConnectInstanceConverter::new(Arc::clone(
        &injectable.connect,
    )));
    injector.register(connect::AwsConnectBotAssociationConverter::new(Arc::clone(
        &injectable.connect,
    )));
    injector.register(connect::AwsConnectContactFlowConverter::new(Arc::clone(
        &injectable.connect,
    )));
    injector.register(connect::AwsConnectContactFlowModuleConverter::new(
        Arc::clone(&injectable.connect),
    ));
    injector.register(connect::AwsConnectHoursOfOperationConverter::new(
        Arc::clone(&injectable.connect),
    ));
    injector.register(connect::AwsConnectInstanceStorageConfigConverter::new(
        Arc::clone(&injectable.connect),
    ));
    injector.register(connect::AwsConnectLambdaFunctionAssociationConverter::new(
        Arc::clone(&injectable.connect),
    ));
    injector.register(connect::AwsConnectPhoneNumberConverter::new(Arc::clone(
        &injectable.connect,
    )));
    injector.register(connect::AwsConnectQueueConverter::new(Arc::clone(
        &injectable.connect,
    )));
    injector.register(connect::AwsConnectQuickConnectConverter::new(Arc::clone(
        &injectable.connect,
    )));
    injector.register(connect::AwsConnectRoutingProfileConverter::new(Arc::clone(
        &injectable.connect,
    )));
    injector.register(connect::AwsConnectSecurityProfileConverter::new(
        Arc::clone(&injectable.connect),
    ));
    injector.register(connect::AwsConnectUserConverter::new(Arc::clone(
        &injectable.connect,
    )));
    injector.register(connect::AwsConnectUserHierarchyGroupConverter::new(
        Arc::clone(&injectable.connect),
    ));
    injector.register(connect::AwsConnectUserHierarchyStructureConverter::new(
        Arc::clone(&injectable.connect),
    ));
    injector.register(connect::AwsConnectVocabularyConverter::new(Arc::clone(
        &injectable.connect,
    )));
    injector.register(costexplorer::AwsCeAnomalyMonitorConverter::new(Arc::clone(
        &injectable.costexplorer,
    )));
    injector.register(costexplorer::AwsCeAnomalySubscriptionConverter::new(
        Arc::clone(&injectable.costexplorer),
    ));
    injector.register(datapipeline::AwsDatapipelinePipelineConverter::new(
        Arc::clone(&injectable.datapipeline),
    ));
    injector.register(datasync::AwsDatasyncTaskConverter::new(Arc::clone(
        &injectable.datasync,
    )));
    injector.register(datasync::AwsDatasyncLocationS3Converter::new(Arc::clone(
        &injectable.datasync,
    )));
    injector.register(datasync::AwsDatasyncAgentConverter::new(Arc::clone(
        &injectable.datasync,
    )));
    injector.register(datasync::AwsDatasyncLocationAzureBlobConverter::new(
        Arc::clone(&injectable.datasync),
    ));
    injector.register(datasync::AwsDatasyncLocationEfsConverter::new(Arc::clone(
        &injectable.datasync,
    )));
    injector.register(
        datasync::AwsDatasyncLocationFsxLustreFileSystemConverter::new(Arc::clone(
            &injectable.datasync,
        )),
    );
    injector.register(
        datasync::AwsDatasyncLocationFsxOntapFileSystemConverter::new(Arc::clone(
            &injectable.datasync,
        )),
    );
    injector.register(
        datasync::AwsDatasyncLocationFsxOpenZfsFileSystemConverter::new(Arc::clone(
            &injectable.datasync,
        )),
    );
    injector.register(
        datasync::AwsDatasyncLocationFsxWindowsFileSystemConverter::new(Arc::clone(
            &injectable.datasync,
        )),
    );
    injector.register(datasync::AwsDatasyncLocationHdfsConverter::new(Arc::clone(
        &injectable.datasync,
    )));
    injector.register(datasync::AwsDatasyncLocationNfsConverter::new(Arc::clone(
        &injectable.datasync,
    )));
    injector.register(datasync::AwsDatasyncLocationObjectStorageConverter::new(
        Arc::clone(&injectable.datasync),
    ));
    injector.register(datasync::AwsDatasyncLocationSmbConverter::new(Arc::clone(
        &injectable.datasync,
    )));
    injector.register(directconnect::AwsDxConnectionConverter::new(Arc::clone(
        &injectable.directconnect,
    )));
    injector.register(directconnect::AwsDxBgpPeerConverter::new(Arc::clone(
        &injectable.directconnect,
    )));
    injector.register(directconnect::AwsDxConnectionAssociationConverter::new(
        Arc::clone(&injectable.directconnect),
    ));
    injector.register(directconnect::AwsDxConnectionConfirmationConverter::new(
        Arc::clone(&injectable.directconnect),
    ));
    injector.register(directconnect::AwsDxGatewayAssociationConverter::new(
        Arc::clone(&injectable.directconnect),
    ));
    injector.register(
        directconnect::AwsDxGatewayAssociationProposalConverter::new(Arc::clone(
            &injectable.directconnect,
        )),
    );
    injector.register(directconnect::AwsDxGatewayConverter::new(Arc::clone(
        &injectable.directconnect,
    )));
    injector.register(directconnect::AwsDxHostedConnectionConverter::new(
        Arc::clone(&injectable.directconnect),
    ));
    injector.register(
        directconnect::AwsDxHostedPrivateVirtualInterfaceAccepterConverter::new(Arc::clone(
            &injectable.directconnect,
        )),
    );
    injector.register(
        directconnect::AwsDxHostedPrivateVirtualInterfaceConverter::new(Arc::clone(
            &injectable.directconnect,
        )),
    );
    injector.register(
        directconnect::AwsDxHostedPublicVirtualInterfaceAccepterConverter::new(Arc::clone(
            &injectable.directconnect,
        )),
    );
    injector.register(
        directconnect::AwsDxHostedPublicVirtualInterfaceConverter::new(Arc::clone(
            &injectable.directconnect,
        )),
    );
    injector.register(
        directconnect::AwsDxHostedTransitVirtualInterfaceAccepterConverter::new(Arc::clone(
            &injectable.directconnect,
        )),
    );
    injector.register(
        directconnect::AwsDxHostedTransitVirtualInterfaceConverter::new(Arc::clone(
            &injectable.directconnect,
        )),
    );
    injector.register(directconnect::AwsDxLagConverter::new(Arc::clone(
        &injectable.directconnect,
    )));
    injector.register(directconnect::AwsDxMacsecKeyAssociationConverter::new(
        Arc::clone(&injectable.directconnect),
    ));
    injector.register(directconnect::AwsDxPrivateVirtualInterfaceConverter::new(
        Arc::clone(&injectable.directconnect),
    ));
    injector.register(directconnect::AwsDxPublicVirtualInterfaceConverter::new(
        Arc::clone(&injectable.directconnect),
    ));
    injector.register(directconnect::AwsDxTransitVirtualInterfaceConverter::new(
        Arc::clone(&injectable.directconnect),
    ));
    injector.register(directory::AwsDirectoryServiceDirectoryConverter::new(
        Arc::clone(&injectable.directory),
    ));
    injector.register(
        directory::AwsDirectoryServiceConditionalForwarderConverter::new(Arc::clone(
            &injectable.directory,
        )),
    );
    injector.register(directory::AwsDirectoryServiceLogSubscriptionConverter::new(
        Arc::clone(&injectable.directory),
    ));
    injector.register(directory::AwsDirectoryServiceRadiusSettingsConverter::new(
        Arc::clone(&injectable.directory),
    ));
    injector.register(directory::AwsDirectoryServiceRegionConverter::new(
        Arc::clone(&injectable.directory),
    ));
    injector.register(
        directory::AwsDirectoryServiceSharedDirectoryAccepterConverter::new(Arc::clone(
            &injectable.directory,
        )),
    );
    injector.register(directory::AwsDirectoryServiceSharedDirectoryConverter::new(
        Arc::clone(&injectable.directory),
    ));
    injector.register(directory::AwsDirectoryServiceTrustConverter::new(
        Arc::clone(&injectable.directory),
    ));
    injector.register(dms::AwsDmsEndpointConverter::new(Arc::clone(
        &injectable.dms,
    )));
    injector.register(dms::AwsDmsReplicationInstanceConverter::new(Arc::clone(
        &injectable.dms,
    )));
    injector.register(dms::AwsDmsReplicationTaskConverter::new(Arc::clone(
        &injectable.dms,
    )));
    injector.register(dsql::AwsDsqlClusterConverter::new(Arc::clone(
        &injectable.dsql,
    )));
    {
        let dynamodb_svc = Arc::clone(&injectable.dynamodb);
        injector.register_with_scopes(
            dynamodb::AwsDynamodbTableConverter::new(Arc::clone(&injectable.dynamodb)),
            move || dynamodb_svc.scopes_with_state(),
        );
    }
    injector.register(dynamodb::AwsDynamodbContributorInsightsConverter::new(
        Arc::clone(&injectable.dynamodb),
    ));
    injector.register(dynamodb::AwsDynamodbGlobalTableConverter::new(Arc::clone(
        &injectable.dynamodb,
    )));
    injector.register(
        dynamodb::AwsDynamodbKinesisStreamingDestinationConverter::new(Arc::clone(
            &injectable.dynamodb,
        )),
    );
    injector.register(dynamodb::AwsDynamodbResourcePolicyConverter::new(
        Arc::clone(&injectable.dynamodb),
    ));
    injector.register(dynamodb::AwsDynamodbTableExportConverter::new(Arc::clone(
        &injectable.dynamodb,
    )));
    injector.register(dynamodb::AwsDynamodbTableItemConverter::new(Arc::clone(
        &injectable.dynamodb,
    )));
    injector.register(dynamodb::AwsDynamodbTableReplicaConverter::new(Arc::clone(
        &injectable.dynamodb,
    )));
    injector.register(dynamodb::AwsDynamodbTagConverter::new(Arc::clone(
        &injectable.dynamodb,
    )));
    injector.register(ec2::AwsVpcConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsSubnetConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsInternetGatewayConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsRouteTableConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsRouteConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsSecurityGroupConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsKeyPairConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsEipConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsNatGatewayConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsPlacementGroupConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsEgressOnlyInternetGatewayConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsNetworkAclConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsNetworkAclRuleConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsCustomerGatewayConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsInstanceConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsLaunchTemplateConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVolumeAttachmentConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsNetworkInterfaceConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsNetworkInterfaceAttachmentConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsNetworkInterfaceSgAttachmentConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEipAssociationConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsInternetGatewayAttachmentConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsFlowLogConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsVpcDhcpOptionsConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcDhcpOptionsAssociationConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsSecurityGroupRuleConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcSecurityGroupIngressRuleConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcSecurityGroupEgressRuleConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcSecurityGroupVpcAssociationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEipDomainNameConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsMainRouteTableAssociationConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsRouteTableAssociationConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsNetworkAclAssociationConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpnGatewayAttachmentConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsAmiConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsAmiCopyConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsAmiFromInstanceConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsAmiLaunchPermissionConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcEndpointConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcEndpointServiceConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcEndpointServiceAllowedPrincipalConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(
        ec2::AwsVpcEndpointServicePrivateDnsVerificationConverter::new(Arc::clone(&injectable.ec2)),
    );
    injector.register(ec2::AwsVpcEndpointSubnetAssociationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcEndpointRouteTableAssociationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcEndpointSecurityGroupAssociationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcEndpointConnectionAccepterConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcEndpointConnectionNotificationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcEndpointPolicyConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcEndpointPrivateDnsConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpnConnectionConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpnConnectionRouteConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpnGatewayConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpnGatewayRoutePropagationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEc2TransitGatewayConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsEc2TransitGatewayRouteTableConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEc2TransitGatewayRouteConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(
        ec2::AwsEc2TransitGatewayRouteTableAssociationConverter::new(Arc::clone(&injectable.ec2)),
    );
    injector.register(
        ec2::AwsEc2TransitGatewayRouteTablePropagationConverter::new(Arc::clone(&injectable.ec2)),
    );
    injector.register(
        ec2::AwsEc2TransitGatewayDefaultRouteTableAssociationConverter::new(Arc::clone(
            &injectable.ec2,
        )),
    );
    injector.register(
        ec2::AwsEc2TransitGatewayDefaultRouteTablePropagationConverter::new(Arc::clone(
            &injectable.ec2,
        )),
    );
    injector.register(ec2::AwsEc2TransitGatewayVpcAttachmentConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(
        ec2::AwsEc2TransitGatewayVpcAttachmentAccepterConverter::new(Arc::clone(&injectable.ec2)),
    );
    injector.register(ec2::AwsEc2TransitGatewayPeeringAttachmentConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(
        ec2::AwsEc2TransitGatewayPeeringAttachmentAccepterConverter::new(Arc::clone(
            &injectable.ec2,
        )),
    );
    injector.register(ec2::AwsEc2TransitGatewayConnectPeerConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(
        ec2::AwsEc2TransitGatewayPolicyTableAssociationConverter::new(Arc::clone(&injectable.ec2)),
    );
    injector.register(ec2::AwsEc2TransitGatewayPrefixListReferenceConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(
        ec2::AwsEc2TransitGatewayMulticastDomainAssociationConverter::new(Arc::clone(
            &injectable.ec2,
        )),
    );
    injector.register(ec2::AwsEc2TransitGatewayMulticastGroupMemberConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEc2TransitGatewayMulticastGroupSourceConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    // Pre-existing TGW sibling converters from earlier waves.
    injector.register(ec2::AwsEc2TransitGatewayMulticastDomainConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEc2TransitGatewayConnectConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsEc2TransitGatewayPolicyTableConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    // Wave 5 — Client VPN family
    injector.register(ec2::AwsEc2ClientVpnEndpointConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsEc2ClientVpnAuthorizationRuleConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEc2ClientVpnNetworkAssociationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEc2ClientVpnRouteConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    // Wave 5 — Host / Fleet / Spot
    injector.register(ec2::AwsEc2HostConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsEc2FleetConverter::new(Arc::clone(&injectable.ec2)));
    injector.register(ec2::AwsSpotFleetRequestConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsSpotInstanceRequestConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsSpotDatafeedSubscriptionConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    // Wave 5 — Carrier Gateway + Managed Prefix List + Tag
    injector.register(ec2::AwsEc2CarrierGatewayConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsEc2ManagedPrefixListConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsEc2ManagedPrefixListEntryConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsEc2TagConverter::new(Arc::clone(&injectable.ec2)));
    // Wave 5 — VPC Peering family
    injector.register(ec2::AwsVpcPeeringConnectionConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcPeeringConnectionAccepterConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcPeeringConnectionOptionsConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    // Wave 6 — Default-* family
    injector.register(ec2::AwsDefaultNetworkAclConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsDefaultRouteTableConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsDefaultSecurityGroupConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsDefaultSubnetConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsDefaultVpcConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsDefaultVpcDhcpOptionsConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    // Wave 6 — Account-level singletons
    injector.register(ec2::AwsEc2AvailabilityZoneGroupConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsEc2DefaultCreditSpecificationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEc2SerialConsoleAccessConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsEc2ImageBlockPublicAccessConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsEc2InstanceMetadataDefaultsConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEc2InstanceStateConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    // Wave 6 — IPAM extras
    injector.register(ec2::AwsVpcIpamOrganizationAdminAccountConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcIpamPreviewNextCidrConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcIpamResourceDiscoveryAssociationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsEc2SubnetCidrReservationConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    // Wave 6 — VPC misc
    injector.register(ec2::AwsVpcBlockPublicAccessExclusionConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcBlockPublicAccessOptionsConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcIpv4CidrBlockAssociationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(ec2::AwsVpcIpv6CidrBlockAssociationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    injector.register(
        ec2::AwsVpcNetworkPerformanceMetricSubscriptionConverter::new(Arc::clone(&injectable.ec2)),
    );
    // Wave 6 — Route server family
    injector.register(ec2::AwsVpcRouteServerConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcRouteServerEndpointConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcRouteServerPeerConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcRouteServerPropagationConverter::new(Arc::clone(
        &injectable.ec2,
    )));
    injector.register(ec2::AwsVpcRouteServerVpcAssociationConverter::new(
        Arc::clone(&injectable.ec2),
    ));
    // Wave 6 — Verified Access extras
    injector.register(
        ec2::AwsVerifiedaccessInstanceLoggingConfigurationConverter::new(Arc::clone(
            &injectable.ec2,
        )),
    );
    injector.register(
        ec2::AwsVerifiedaccessInstanceTrustProviderAttachmentConverter::new(Arc::clone(
            &injectable.ec2,
        )),
    );
    injector.register(ecr::AwsEcrRepositoryConverter::new(Arc::clone(
        &injectable.ecr,
    )));
    injector.register(ecr::AwsEcrAccountSettingConverter::new(Arc::clone(
        &injectable.ecr,
    )));
    injector.register(ecr::AwsEcrLifecyclePolicyConverter::new(Arc::clone(
        &injectable.ecr,
    )));
    injector.register(ecr::AwsEcrPullThroughCacheRuleConverter::new(Arc::clone(
        &injectable.ecr,
    )));
    injector.register(ecr::AwsEcrRegistryPolicyConverter::new(Arc::clone(
        &injectable.ecr,
    )));
    injector.register(ecr::AwsEcrRegistryScanningConfigurationConverter::new(
        Arc::clone(&injectable.ecr),
    ));
    injector.register(ecr::AwsEcrReplicationConfigurationConverter::new(
        Arc::clone(&injectable.ecr),
    ));
    injector.register(ecr::AwsEcrRepositoryCreationTemplateConverter::new(
        Arc::clone(&injectable.ecr),
    ));
    injector.register(ecr::AwsEcrRepositoryPolicyConverter::new(Arc::clone(
        &injectable.ecr,
    )));
    {
        let mk_sp = || {
            let svc = Arc::clone(&injectable.ecs);
            move || svc.scopes_with_state()
        };
        injector.register_with_scopes(
            ecs::AwsEcsClusterConverter::new(Arc::clone(&injectable.ecs)),
            mk_sp(),
        );
        injector.register_with_scopes(
            ecs::AwsEcsTaskDefinitionConverter::new(Arc::clone(&injectable.ecs)),
            mk_sp(),
        );
        injector.register_with_scopes(
            ecs::AwsEcsServiceConverter::new(Arc::clone(&injectable.ecs)),
            mk_sp(),
        );
    }
    injector.register(efs::AwsEfsFileSystemConverter::new(Arc::clone(
        &injectable.efs,
    )));
    {
        let mk_sp = || {
            let svc = Arc::clone(&injectable.eks);
            move || svc.scopes_with_state()
        };
        injector.register_with_scopes(
            eks::AwsEksClusterConverter::new(Arc::clone(&injectable.eks)),
            mk_sp(),
        );
        injector.register_with_scopes(
            eks::AwsEksNodeGroupConverter::new(Arc::clone(&injectable.eks)),
            mk_sp(),
        );
    }
    injector.register(eks::AwsEksAccessEntryConverter::new(Arc::clone(
        &injectable.eks,
    )));
    injector.register(eks::AwsEksAccessPolicyAssociationConverter::new(
        Arc::clone(&injectable.eks),
    ));
    injector.register(eks::AwsEksAddonConverter::new(Arc::clone(&injectable.eks)));
    injector.register(eks::AwsEksFargateProfileConverter::new(Arc::clone(
        &injectable.eks,
    )));
    injector.register(eks::AwsEksIdentityProviderConfigConverter::new(Arc::clone(
        &injectable.eks,
    )));
    injector.register(eks::AwsEksPodIdentityAssociationConverter::new(Arc::clone(
        &injectable.eks,
    )));
    injector.register(elasticache::AwsElastiCacheClusterConverter::new(
        Arc::clone(&injectable.elasticache),
    ));
    injector.register(elasticache::AwsElastiCacheReplicationGroupConverter::new(
        Arc::clone(&injectable.elasticache),
    ));
    injector.register(elasticache::AwsElastiCacheSubnetGroupConverter::new(
        Arc::clone(&injectable.elasticache),
    ));
    injector.register(elasticache::AwsElastiCacheParameterGroupConverter::new(
        Arc::clone(&injectable.elasticache),
    ));
    injector.register(
        elasticache::AwsElastiCacheGlobalReplicationGroupConverter::new(Arc::clone(
            &injectable.elasticache,
        )),
    );
    injector.register(elasticache::AwsElastiCacheReservedCacheNodeConverter::new(
        Arc::clone(&injectable.elasticache),
    ));
    injector.register(elasticache::AwsElastiCacheServerlessCacheConverter::new(
        Arc::clone(&injectable.elasticache),
    ));
    injector.register(elasticache::AwsElastiCacheUserConverter::new(Arc::clone(
        &injectable.elasticache,
    )));
    injector.register(
        elasticache::AwsElastiCacheUserGroupAssociationConverter::new(Arc::clone(
            &injectable.elasticache,
        )),
    );
    injector.register(elasticache::AwsElastiCacheUserGroupConverter::new(
        Arc::clone(&injectable.elasticache),
    ));
    injector.register(elasticloadbalancing::AwsElbConverter::new(Arc::clone(
        &injectable.elasticloadbalancing,
    )));
    injector.register(elbv2::AwsLbConverter::new(Arc::clone(&injectable.elbv2)));
    injector.register(elbv2::AwsLbTargetGroupConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsLbListenerConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsAlbConverter::new(Arc::clone(&injectable.elbv2)));
    injector.register(elbv2::AwsAlbListenerCertificateConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsAlbListenerConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsAlbListenerRuleConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsAlbTargetGroupAttachmentConverter::new(
        Arc::clone(&injectable.elbv2),
    ));
    injector.register(elbv2::AwsAlbTargetGroupConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsLbCookieStickinessPolicyConverter::new(
        Arc::clone(&injectable.elbv2),
    ));
    injector.register(elbv2::AwsLbListenerCertificateConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsLbListenerRuleConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsLbSslNegotiationPolicyConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsLbTargetGroupAttachmentConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsLbTrustStoreConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(elbv2::AwsLbTrustStoreRevocationConverter::new(Arc::clone(
        &injectable.elbv2,
    )));
    injector.register(emr::AwsEmrClusterConverter::new(Arc::clone(
        &injectable.emr,
    )));
    injector.register(emr::AwsEmrSecurityConfigurationConverter::new(Arc::clone(
        &injectable.emr,
    )));
    injector.register(emr::AwsEmrBlockPublicAccessConfigurationConverter::new(
        Arc::clone(&injectable.emr),
    ));
    injector.register(emr::AwsEmrInstanceFleetConverter::new(Arc::clone(
        &injectable.emr,
    )));
    injector.register(emr::AwsEmrInstanceGroupConverter::new(Arc::clone(
        &injectable.emr,
    )));
    injector.register(emr::AwsEmrManagedScalingPolicyConverter::new(Arc::clone(
        &injectable.emr,
    )));
    injector.register(emr::AwsEmrStudioConverter::new(Arc::clone(&injectable.emr)));
    injector.register(emr::AwsEmrStudioSessionMappingConverter::new(Arc::clone(
        &injectable.emr,
    )));
    injector.register(emrcontainers::AwsEmrcontainersVirtualClusterConverter::new(
        Arc::clone(&injectable.emrcontainers),
    ));
    injector.register(emrserverless::AwsEmrserverlessApplicationConverter::new(
        Arc::clone(&injectable.emrserverless),
    ));
    injector.register(events::AwsCloudwatchEventBusConverter::new(Arc::clone(
        &injectable.events,
    )));
    injector.register(events::AwsCloudwatchEventRuleConverter::new(Arc::clone(
        &injectable.events,
    )));
    injector.register(events::AwsCloudwatchEventTargetConverter::new(Arc::clone(
        &injectable.events,
    )));
    injector.register(events::AwsCloudwatchEventApiDestinationConverter::new(
        Arc::clone(&injectable.events),
    ));
    injector.register(events::AwsCloudwatchEventArchiveConverter::new(Arc::clone(
        &injectable.events,
    )));
    injector.register(events::AwsCloudwatchEventBusPolicyConverter::new(
        Arc::clone(&injectable.events),
    ));
    injector.register(events::AwsCloudwatchEventConnectionConverter::new(
        Arc::clone(&injectable.events),
    ));
    injector.register(events::AwsCloudwatchEventEndpointConverter::new(
        Arc::clone(&injectable.events),
    ));
    injector.register(events::AwsCloudwatchEventPermissionConverter::new(
        Arc::clone(&injectable.events),
    ));
    injector.register(firehose::AwsKinesisFirehoseDeliveryStreamConverter::new(
        Arc::clone(&injectable.firehose),
    ));
    injector.register(fis::AwsFisExperimentTemplateConverter::new(Arc::clone(
        &injectable.fis,
    )));
    injector.register(fsx::AwsFsxBackupConverter::new(Arc::clone(&injectable.fsx)));
    injector.register(fsx::AwsFsxDataRepositoryAssociationConverter::new(
        Arc::clone(&injectable.fsx),
    ));
    injector.register(fsx::AwsFsxFileCacheConverter::new(Arc::clone(
        &injectable.fsx,
    )));
    injector.register(fsx::AwsFsxLustreFileSystemConverter::new(Arc::clone(
        &injectable.fsx,
    )));
    injector.register(fsx::AwsFsxOntapFileSystemConverter::new(Arc::clone(
        &injectable.fsx,
    )));
    injector.register(fsx::AwsFsxOntapStorageVirtualMachineConverter::new(
        Arc::clone(&injectable.fsx),
    ));
    injector.register(fsx::AwsFsxOntapVolumeConverter::new(Arc::clone(
        &injectable.fsx,
    )));
    injector.register(fsx::AwsFsxOpenzfsFileSystemConverter::new(Arc::clone(
        &injectable.fsx,
    )));
    injector.register(fsx::AwsFsxOpenzfsSnapshotConverter::new(Arc::clone(
        &injectable.fsx,
    )));
    injector.register(fsx::AwsFsxOpenzfsVolumeConverter::new(Arc::clone(
        &injectable.fsx,
    )));
    injector.register(fsx::AwsFsxWindowsFileSystemConverter::new(Arc::clone(
        &injectable.fsx,
    )));
    injector.register(glacier::AwsGlacierVaultConverter::new(Arc::clone(
        &injectable.glacier,
    )));
    injector.register(glue::AwsGlueCatalogDatabaseConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGlueJobConverter::new(Arc::clone(&injectable.glue)));
    injector.register(glue::AwsGlueCrawlerConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGlueCatalogTableConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGlueConnectionConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGlueDataCatalogEncryptionSettingsConverter::new(
        Arc::clone(&injectable.glue),
    ));
    injector.register(glue::AwsGlueDevEndpointConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGlueMlTransformConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGluePartitionConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGlueRegistryConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGlueResourcePolicyConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGlueSchemaConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGlueSecurityConfigurationConverter::new(
        Arc::clone(&injectable.glue),
    ));
    injector.register(glue::AwsGlueTriggerConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(glue::AwsGlueWorkflowConverter::new(Arc::clone(
        &injectable.glue,
    )));
    injector.register(guardduty::AwsGuarddutyDetectorConverter::new(Arc::clone(
        &injectable.guardduty,
    )));
    injector.register(guardduty::AwsGuarddutyFilterConverter::new(Arc::clone(
        &injectable.guardduty,
    )));
    injector.register(guardduty::AwsGuarddutyMemberConverter::new(Arc::clone(
        &injectable.guardduty,
    )));
    injector.register(guardduty::AwsGuarddutyDetectorFeatureConverter::new(
        Arc::clone(&injectable.guardduty),
    ));
    injector.register(guardduty::AwsGuarddutyInviteAccepterConverter::new(
        Arc::clone(&injectable.guardduty),
    ));
    injector.register(guardduty::AwsGuarddutyIpsetConverter::new(Arc::clone(
        &injectable.guardduty,
    )));
    injector.register(guardduty::AwsGuarddutyMalwareProtectionPlanConverter::new(
        Arc::clone(&injectable.guardduty),
    ));
    injector.register(guardduty::AwsGuarddutyMemberDetectorFeatureConverter::new(
        Arc::clone(&injectable.guardduty),
    ));
    injector.register(
        guardduty::AwsGuarddutyOrganizationAdminAccountConverter::new(Arc::clone(
            &injectable.guardduty,
        )),
    );
    injector.register(
        guardduty::AwsGuarddutyOrganizationConfigurationConverter::new(Arc::clone(
            &injectable.guardduty,
        )),
    );
    injector.register(
        guardduty::AwsGuarddutyOrganizationConfigurationFeatureConverter::new(Arc::clone(
            &injectable.guardduty,
        )),
    );
    injector.register(guardduty::AwsGuarddutyPublishingDestinationConverter::new(
        Arc::clone(&injectable.guardduty),
    ));
    injector.register(guardduty::AwsGuarddutyThreatintelsetConverter::new(
        Arc::clone(&injectable.guardduty),
    ));
    injector.register(iam::AwsIamUserConverter::new(Arc::clone(&injectable.iam)));
    injector.register(iam::AwsIamRoleConverter::new(Arc::clone(&injectable.iam)));
    injector.register(iam::AwsIamPolicyConverter::new(Arc::clone(&injectable.iam)));
    injector.register(iam::AwsIamGroupConverter::new(Arc::clone(&injectable.iam)));
    injector.register(iam::AwsIamInstanceProfileConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamRolePolicyAttachmentConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamUserPolicyAttachmentConverter::new(Arc::clone(
        &injectable.iam,
    )));
    // Wave 1: top-level IAM resources
    injector.register(iam::AwsIamOpenidConnectProviderConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamSamlProviderConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamVirtualMfaDeviceConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamServerCertificateConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamSigningCertificateConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamServiceSpecificCredentialConverter::new(
        Arc::clone(&injectable.iam),
    ));
    injector.register(iam::AwsIamUserSshKeyConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamAccessKeyConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamUserLoginProfileConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamAccountPasswordPolicyConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamAccountAliasConverter::new(Arc::clone(
        &injectable.iam,
    )));
    // Wave 2: sub-resource modifiers
    injector.register(iam::AwsIamRolePolicyConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamUserPolicyConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamGroupPolicyConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamGroupPolicyAttachmentConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamGroupMembershipConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamUserGroupMembershipConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamServiceLinkedRoleConverter::new(Arc::clone(
        &injectable.iam,
    )));
    // Wave 3: multi-target + exclusives
    injector.register(iam::AwsIamPolicyAttachmentConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamRolePoliciesExclusiveConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamUserPoliciesExclusiveConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamGroupPoliciesExclusiveConverter::new(Arc::clone(
        &injectable.iam,
    )));
    injector.register(iam::AwsIamRolePolicyAttachmentsExclusiveConverter::new(
        Arc::clone(&injectable.iam),
    ));
    injector.register(iam::AwsIamUserPolicyAttachmentsExclusiveConverter::new(
        Arc::clone(&injectable.iam),
    ));
    injector.register(iam::AwsIamGroupPolicyAttachmentsExclusiveConverter::new(
        Arc::clone(&injectable.iam),
    ));
    injector.register(identitystore::AwsIdentitystoreGroupConverter::new(
        Arc::clone(&injectable.identitystore),
    ));
    injector.register(identitystore::AwsIdentitystoreUserConverter::new(
        Arc::clone(&injectable.identitystore),
    ));
    injector.register(inspector2::AwsInspector2EnablerConverter::new(Arc::clone(
        &injectable.inspector2,
    )));
    injector.register(iot::AwsIotThingConverter::new(Arc::clone(&injectable.iot)));
    injector.register(iot::AwsIotThingTypeConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotPolicyConverter::new(Arc::clone(&injectable.iot)));
    injector.register(iot::AwsIotCertificateConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotAuthorizerConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotBillingGroupConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotCaCertificateConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotDomainConfigurationConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotEventConfigurationsConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotIndexingConfigurationConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotLoggingOptionsConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotPolicyAttachmentConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotProvisioningTemplateConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotRoleAliasConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotThingGroupConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotThingGroupMembershipConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotThingPrincipalAttachmentConverter::new(
        Arc::clone(&injectable.iot),
    ));
    injector.register(iot::AwsIotTopicRuleConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(iot::AwsIotTopicRuleDestinationConverter::new(Arc::clone(
        &injectable.iot,
    )));
    injector.register(ivs::AwsIvsChannelConverter::new(Arc::clone(
        &injectable.ivs,
    )));
    injector.register(kafka::AwsMskClusterConverter::new(Arc::clone(
        &injectable.kafka,
    )));
    injector.register(kafka::AwsMskClusterPolicyConverter::new(Arc::clone(
        &injectable.kafka,
    )));
    injector.register(kafka::AwsMskConfigurationConverter::new(Arc::clone(
        &injectable.kafka,
    )));
    injector.register(kafka::AwsMskReplicatorConverter::new(Arc::clone(
        &injectable.kafka,
    )));
    injector.register(kafka::AwsMskScramSecretAssociationConverter::new(
        Arc::clone(&injectable.kafka),
    ));
    injector.register(kafka::AwsMskServerlessClusterConverter::new(Arc::clone(
        &injectable.kafka,
    )));
    injector.register(kafka::AwsMskSingleScramSecretAssociationConverter::new(
        Arc::clone(&injectable.kafka),
    ));
    injector.register(kafka::AwsMskVpcConnectionConverter::new(Arc::clone(
        &injectable.kafka,
    )));
    injector.register(kinesis::AwsKinesisStreamConverter::new(Arc::clone(
        &injectable.kinesis,
    )));
    injector.register(
        kinesisanalyticsv2::AwsKinesisanalyticsv2ApplicationConverter::new(Arc::clone(
            &injectable.kinesisanalyticsv2,
        )),
    );
    injector.register(kinesisvideo::AwsKinesisVideoStreamConverter::new(
        Arc::clone(&injectable.kinesisvideo),
    ));
    injector.register(kms::AwsKmsKeyConverter::new(Arc::clone(&injectable.kms)));
    injector.register(kms::AwsKmsAliasConverter::new(Arc::clone(&injectable.kms)));
    injector.register(kms::AwsKmsCiphertextConverter::new(Arc::clone(
        &injectable.kms,
    )));
    injector.register(kms::AwsKmsCustomKeyStoreConverter::new(Arc::clone(
        &injectable.kms,
    )));
    injector.register(kms::AwsKmsExternalKeyConverter::new(Arc::clone(
        &injectable.kms,
    )));
    injector.register(kms::AwsKmsGrantConverter::new(Arc::clone(&injectable.kms)));
    injector.register(kms::AwsKmsKeyPolicyConverter::new(Arc::clone(
        &injectable.kms,
    )));
    injector.register(kms::AwsKmsReplicaExternalKeyConverter::new(Arc::clone(
        &injectable.kms,
    )));
    injector.register(kms::AwsKmsReplicaKeyConverter::new(Arc::clone(
        &injectable.kms,
    )));
    injector.register(lakeformation::AwsLakeformationResourceConverter::new(
        Arc::clone(&injectable.lakeformation),
    ));
    injector.register(
        lakeformation::AwsLakeformationDataLakeSettingsConverter::new(Arc::clone(
            &injectable.lakeformation,
        )),
    );
    injector.register(
        lakeformation::AwsLakeformationDataCellsFilterConverter::new(Arc::clone(
            &injectable.lakeformation,
        )),
    );
    injector.register(lakeformation::AwsLakeformationLfTagConverter::new(
        Arc::clone(&injectable.lakeformation),
    ));
    injector.register(lakeformation::AwsLakeformationOptInConverter::new(
        Arc::clone(&injectable.lakeformation),
    ));
    injector.register(lakeformation::AwsLakeformationPermissionsConverter::new(
        Arc::clone(&injectable.lakeformation),
    ));
    injector.register(lakeformation::AwsLakeformationResourceLfTagConverter::new(
        Arc::clone(&injectable.lakeformation),
    ));
    injector.register(lakeformation::AwsLakeformationResourceLfTagsConverter::new(
        Arc::clone(&injectable.lakeformation),
    ));
    {
        let mk_sp = || {
            let svc = Arc::clone(&injectable.lambda);
            move || svc.scopes_with_state()
        };
        injector.register_with_scopes(
            lambda::AwsLambdaFunctionConverter::new(Arc::clone(&injectable.lambda)),
            mk_sp(),
        );
        injector.register_with_scopes(
            lambda::AwsLambdaAliasConverter::new(Arc::clone(&injectable.lambda)),
            mk_sp(),
        );
        injector.register_with_scopes(
            lambda::AwsLambdaPermissionConverter::new(Arc::clone(&injectable.lambda)),
            mk_sp(),
        );
        injector.register_with_scopes(
            lambda::AwsLambdaEventSourceMappingConverter::new(Arc::clone(&injectable.lambda)),
            mk_sp(),
        );
    }
    injector.register(lambda::AwsLambdaCodeSigningConfigConverter::new(
        Arc::clone(&injectable.lambda),
    ));
    injector.register(lambda::AwsLambdaFunctionEventInvokeConfigConverter::new(
        Arc::clone(&injectable.lambda),
    ));
    injector.register(lambda::AwsLambdaFunctionRecursionConfigConverter::new(
        Arc::clone(&injectable.lambda),
    ));
    injector.register(lambda::AwsLambdaFunctionUrlConverter::new(Arc::clone(
        &injectable.lambda,
    )));
    injector.register(lambda::AwsLambdaInvocationConverter::new(Arc::clone(
        &injectable.lambda,
    )));
    injector.register(lambda::AwsLambdaLayerVersionConverter::new(Arc::clone(
        &injectable.lambda,
    )));
    injector.register(lambda::AwsLambdaLayerVersionPermissionConverter::new(
        Arc::clone(&injectable.lambda),
    ));
    injector.register(lambda::AwsLambdaProvisionedConcurrencyConfigConverter::new(
        Arc::clone(&injectable.lambda),
    ));
    injector.register(lambda::AwsLambdaRuntimeManagementConfigConverter::new(
        Arc::clone(&injectable.lambda),
    ));
    injector.register(lexmodelsv2::AwsLexv2modelsBotConverter::new(Arc::clone(
        &injectable.lexmodelsv2,
    )));
    injector.register(logs::AwsCloudwatchLogGroupConverter::new(Arc::clone(
        &injectable.logs,
    )));
    injector.register(logs::AwsCloudwatchLogStreamConverter::new(Arc::clone(
        &injectable.logs,
    )));
    injector.register(logs::AwsCloudwatchLogAccountPolicyConverter::new(
        Arc::clone(&injectable.logs),
    ));
    injector.register(logs::AwsCloudwatchLogAnomalyDetectorConverter::new(
        Arc::clone(&injectable.logs),
    ));
    injector.register(logs::AwsCloudwatchLogDataProtectionPolicyConverter::new(
        Arc::clone(&injectable.logs),
    ));
    injector.register(logs::AwsCloudwatchLogDeliveryConverter::new(Arc::clone(
        &injectable.logs,
    )));
    injector.register(logs::AwsCloudwatchLogDeliveryDestinationConverter::new(
        Arc::clone(&injectable.logs),
    ));
    injector.register(
        logs::AwsCloudwatchLogDeliveryDestinationPolicyConverter::new(Arc::clone(&injectable.logs)),
    );
    injector.register(logs::AwsCloudwatchLogDeliverySourceConverter::new(
        Arc::clone(&injectable.logs),
    ));
    injector.register(logs::AwsCloudwatchLogDestinationConverter::new(Arc::clone(
        &injectable.logs,
    )));
    injector.register(logs::AwsCloudwatchLogDestinationPolicyConverter::new(
        Arc::clone(&injectable.logs),
    ));
    injector.register(logs::AwsCloudwatchLogIndexPolicyConverter::new(Arc::clone(
        &injectable.logs,
    )));
    injector.register(logs::AwsCloudwatchLogMetricFilterConverter::new(
        Arc::clone(&injectable.logs),
    ));
    injector.register(logs::AwsCloudwatchLogResourcePolicyConverter::new(
        Arc::clone(&injectable.logs),
    ));
    injector.register(logs::AwsCloudwatchLogSubscriptionFilterConverter::new(
        Arc::clone(&injectable.logs),
    ));
    injector.register(macie2::AwsMacie2AccountConverter::new(Arc::clone(
        &injectable.macie2,
    )));
    injector.register(macie2::AwsMacie2ClassificationJobConverter::new(
        Arc::clone(&injectable.macie2),
    ));
    injector.register(
        macie2::AwsMacie2ClassificationExportConfigurationConverter::new(Arc::clone(
            &injectable.macie2,
        )),
    );
    injector.register(macie2::AwsMacie2CustomDataIdentifierConverter::new(
        Arc::clone(&injectable.macie2),
    ));
    injector.register(macie2::AwsMacie2FindingsFilterConverter::new(Arc::clone(
        &injectable.macie2,
    )));
    injector.register(macie2::AwsMacie2InvitationAccepterConverter::new(
        Arc::clone(&injectable.macie2),
    ));
    injector.register(macie2::AwsMacie2MemberConverter::new(Arc::clone(
        &injectable.macie2,
    )));
    injector.register(macie2::AwsMacie2OrganizationAdminAccountConverter::new(
        Arc::clone(&injectable.macie2),
    ));
    injector.register(macie2::AwsMacie2OrganizationConfigurationConverter::new(
        Arc::clone(&injectable.macie2),
    ));
    injector.register(medialive::AwsMedialiveChannelConverter::new(Arc::clone(
        &injectable.medialive,
    )));
    injector.register(medialive::AwsMedialiveInputConverter::new(Arc::clone(
        &injectable.medialive,
    )));
    injector.register(mediapackage::AwsMediaPackageChannelConverter::new(
        Arc::clone(&injectable.mediapackage),
    ));
    injector.register(mediapackagev2::AwsMediaPackageV2ChannelGroupConverter::new(
        Arc::clone(&injectable.mediapackagev2),
    ));
    injector.register(mediastore::AwsMediaStoreContainerConverter::new(
        Arc::clone(&injectable.mediastore),
    ));
    injector.register(mq::AwsMqBrokerConverter::new(Arc::clone(&injectable.mq)));
    injector.register(mq::AwsMqConfigurationConverter::new(Arc::clone(
        &injectable.mq,
    )));
    injector.register(networkfirewall::AwsNetworkFirewallFirewallConverter::new(
        Arc::clone(&injectable.networkfirewall),
    ));
    injector.register(
        networkfirewall::AwsNetworkFirewallFirewallPolicyConverter::new(Arc::clone(
            &injectable.networkfirewall,
        )),
    );
    injector.register(networkfirewall::AwsNetworkFirewallRuleGroupConverter::new(
        Arc::clone(&injectable.networkfirewall),
    ));
    injector.register(
        networkmanager::AwsNetworkmanagerGlobalNetworkConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(networkmanager::AwsNetworkmanagerSiteConverter::new(
        Arc::clone(&injectable.networkmanager),
    ));
    injector.register(networkmanager::AwsNetworkmanagerDeviceConverter::new(
        Arc::clone(&injectable.networkmanager),
    ));
    injector.register(
        networkmanager::AwsNetworkmanagerAttachmentAccepterConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(
        networkmanager::AwsNetworkmanagerConnectAttachmentConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(networkmanager::AwsNetworkmanagerConnectPeerConverter::new(
        Arc::clone(&injectable.networkmanager),
    ));
    injector.register(networkmanager::AwsNetworkmanagerConnectionConverter::new(
        Arc::clone(&injectable.networkmanager),
    ));
    injector.register(networkmanager::AwsNetworkmanagerCoreNetworkConverter::new(
        Arc::clone(&injectable.networkmanager),
    ));
    injector.register(
        networkmanager::AwsNetworkmanagerCoreNetworkPolicyAttachmentConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(
        networkmanager::AwsNetworkmanagerCustomerGatewayAssociationConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(
        networkmanager::AwsNetworkmanagerDxGatewayAttachmentConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(
        networkmanager::AwsNetworkmanagerLinkAssociationConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(networkmanager::AwsNetworkmanagerLinkConverter::new(
        Arc::clone(&injectable.networkmanager),
    ));
    injector.register(
        networkmanager::AwsNetworkmanagerSiteToSiteVpnAttachmentConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(
        networkmanager::AwsNetworkmanagerTransitGatewayConnectPeerAssociationConverter::new(
            Arc::clone(&injectable.networkmanager),
        ),
    );
    injector.register(
        networkmanager::AwsNetworkmanagerTransitGatewayPeeringConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(
        networkmanager::AwsNetworkmanagerTransitGatewayRegistrationConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(
        networkmanager::AwsNetworkmanagerTransitGatewayRouteTableAttachmentConverter::new(
            Arc::clone(&injectable.networkmanager),
        ),
    );
    injector.register(
        networkmanager::AwsNetworkmanagerVpcAttachmentConverter::new(Arc::clone(
            &injectable.networkmanager,
        )),
    );
    injector.register(opensearch::AwsOpensearchDomainConverter::new(Arc::clone(
        &injectable.opensearch,
    )));
    injector.register(
        opensearch::AwsOpensearchAuthorizeVpcEndpointAccessConverter::new(Arc::clone(
            &injectable.opensearch,
        )),
    );
    injector.register(opensearch::AwsOpensearchDomainPolicyConverter::new(
        Arc::clone(&injectable.opensearch),
    ));
    injector.register(opensearch::AwsOpensearchDomainSamlOptionsConverter::new(
        Arc::clone(&injectable.opensearch),
    ));
    injector.register(
        opensearch::AwsOpensearchInboundConnectionAccepterConverter::new(Arc::clone(
            &injectable.opensearch,
        )),
    );
    injector.register(opensearch::AwsOpensearchOutboundConnectionConverter::new(
        Arc::clone(&injectable.opensearch),
    ));
    injector.register(opensearch::AwsOpensearchPackageAssociationConverter::new(
        Arc::clone(&injectable.opensearch),
    ));
    injector.register(opensearch::AwsOpensearchPackageConverter::new(Arc::clone(
        &injectable.opensearch,
    )));
    injector.register(opensearch::AwsOpensearchVpcEndpointConverter::new(
        Arc::clone(&injectable.opensearch),
    ));
    injector.register(
        opensearchserverless::AwsOpensearchserverlessCollectionConverter::new(Arc::clone(
            &injectable.opensearchserverless,
        )),
    );
    injector.register(
        opensearchserverless::AwsOpensearchserverlessSecurityPolicyConverter::new(Arc::clone(
            &injectable.opensearchserverless,
        )),
    );
    injector.register(organizations::AwsOrganizationsAccountConverter::new(
        Arc::clone(&injectable.organizations),
    ));
    injector.register(organizations::AwsOrganizationsOuConverter::new(Arc::clone(
        &injectable.organizations,
    )));
    injector.register(organizations::AwsOrganizationsPolicyConverter::new(
        Arc::clone(&injectable.organizations),
    ));
    injector.register(osis::AwsOsisPipelineConverter::new(Arc::clone(
        &injectable.osis,
    )));
    injector.register(outposts::AwsOutpostsSiteConverter::new(Arc::clone(
        &injectable.outposts,
    )));
    injector.register(outposts::AwsOutpostsOutpostConverter::new(Arc::clone(
        &injectable.outposts,
    )));
    injector.register(keyspaces::AwsKeyspacesKeyspaceConverter::new(Arc::clone(
        &injectable.keyspaces,
    )));
    injector.register(keyspaces::AwsKeyspacesTableConverter::new(Arc::clone(
        &injectable.keyspaces,
    )));
    injector.register(rolesanywhere::AwsRolesAnywhereProfileConverter::new(
        Arc::clone(&injectable.rolesanywhere),
    ));
    injector.register(rolesanywhere::AwsRolesAnywhereTrustAnchorConverter::new(
        Arc::clone(&injectable.rolesanywhere),
    ));
    injector.register(pinpoint::AwsPinpointAdmChannelConverter::new(Arc::clone(
        &injectable.pinpoint,
    )));
    injector.register(pinpoint::AwsPinpointApnsChannelConverter::new(Arc::clone(
        &injectable.pinpoint,
    )));
    injector.register(pinpoint::AwsPinpointApnsSandboxChannelConverter::new(
        Arc::clone(&injectable.pinpoint),
    ));
    injector.register(pinpoint::AwsPinpointApnsVoipChannelConverter::new(
        Arc::clone(&injectable.pinpoint),
    ));
    injector.register(pinpoint::AwsPinpointApnsVoipSandboxChannelConverter::new(
        Arc::clone(&injectable.pinpoint),
    ));
    injector.register(pinpoint::AwsPinpointBaiduChannelConverter::new(Arc::clone(
        &injectable.pinpoint,
    )));
    injector.register(pinpoint::AwsPinpointEmailTemplateConverter::new(
        Arc::clone(&injectable.pinpoint),
    ));
    injector.register(pinpoint::AwsPinpointEventStreamConverter::new(Arc::clone(
        &injectable.pinpoint,
    )));
    injector.register(pinpoint::AwsPinpointGcmChannelConverter::new(Arc::clone(
        &injectable.pinpoint,
    )));
    injector.register(pinpoint::AwsPinpointSmsChannelConverter::new(Arc::clone(
        &injectable.pinpoint,
    )));
    injector.register(pipes::AwsPipesPipeConverter::new(Arc::clone(
        &injectable.pipes,
    )));
    injector.register(quicksight::AwsQuicksightDataSourceConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightGroupConverter::new(Arc::clone(
        &injectable.quicksight,
    )));
    injector.register(quicksight::AwsQuicksightUserConverter::new(Arc::clone(
        &injectable.quicksight,
    )));
    injector.register(quicksight::AwsQuicksightAccountSettingsConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightAccountSubscriptionConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightAnalysisConverter::new(Arc::clone(
        &injectable.quicksight,
    )));
    injector.register(quicksight::AwsQuicksightDashboardConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightDataSetConverter::new(Arc::clone(
        &injectable.quicksight,
    )));
    injector.register(quicksight::AwsQuicksightFolderConverter::new(Arc::clone(
        &injectable.quicksight,
    )));
    injector.register(quicksight::AwsQuicksightFolderMembershipConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightGroupMembershipConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightIamPolicyAssignmentConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightIngestionConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightNamespaceConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightRefreshScheduleConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightRoleMembershipConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightTemplateConverter::new(Arc::clone(
        &injectable.quicksight,
    )));
    injector.register(quicksight::AwsQuicksightTemplateAliasConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(quicksight::AwsQuicksightThemeConverter::new(Arc::clone(
        &injectable.quicksight,
    )));
    injector.register(quicksight::AwsQuicksightVpcConnectionConverter::new(
        Arc::clone(&injectable.quicksight),
    ));
    injector.register(ram::AwsRamResourceShareConverter::new(Arc::clone(
        &injectable.ram,
    )));
    {
        let mk_sp = || {
            let svc = Arc::clone(&injectable.rds);
            move || svc.scopes_with_state()
        };
        injector.register_with_scopes(
            rds::AwsDbSubnetGroupConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbParameterGroupConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsRdsClusterParameterGroupConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbOptionGroupConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbInstanceConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsRdsClusterConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbEventSubscriptionConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbProxyConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbProxyDefaultTargetGroupConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbProxyEndpointConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbProxyTargetConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbSnapshotConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbSnapshotCopyConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsDbClusterSnapshotConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsRdsClusterSnapshotCopyConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsRdsClusterEndpointConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsRdsClusterInstanceConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsRdsExportTaskConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsRdsGlobalClusterConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsRdsShardGroupConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
        injector.register_with_scopes(
            rds::AwsRdsInstanceStateConverter::new(Arc::clone(&injectable.rds)),
            mk_sp(),
        );
    }
    injector.register(rds::AwsDbInstanceAutomatedBackupsReplicationConverter::new(
        Arc::clone(&injectable.rds),
    ));
    injector.register(rds::AwsDbInstanceRoleAssociationConverter::new(Arc::clone(
        &injectable.rds,
    )));
    injector.register(rds::AwsRdsCertificateConverter::new(Arc::clone(
        &injectable.rds,
    )));
    injector.register(rds::AwsRdsClusterActivityStreamConverter::new(Arc::clone(
        &injectable.rds,
    )));
    injector.register(rds::AwsRdsClusterRoleAssociationConverter::new(Arc::clone(
        &injectable.rds,
    )));
    injector.register(rds::AwsRdsCustomDbEngineVersionConverter::new(Arc::clone(
        &injectable.rds,
    )));
    injector.register(rds::AwsRdsIntegrationConverter::new(Arc::clone(
        &injectable.rds,
    )));
    injector.register(rds::AwsRdsReservedInstanceConverter::new(Arc::clone(
        &injectable.rds,
    )));
    injector.register(redshift::AwsRedshiftClusterConverter::new(Arc::clone(
        &injectable.redshift,
    )));
    injector.register(redshift::AwsRedshiftSubnetGroupConverter::new(Arc::clone(
        &injectable.redshift,
    )));
    injector.register(redshift::AwsRedshiftAuthenticationProfileConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftClusterIamRolesConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftClusterSnapshotConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftEventSubscriptionConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftHsmClientCertificateConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftHsmConfigurationConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftLoggingConverter::new(Arc::clone(
        &injectable.redshift,
    )));
    injector.register(redshift::AwsRedshiftParameterGroupConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftPartnerConverter::new(Arc::clone(
        &injectable.redshift,
    )));
    injector.register(redshift::AwsRedshiftResourcePolicyConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftScheduledActionConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftSnapshotCopyConverter::new(Arc::clone(
        &injectable.redshift,
    )));
    injector.register(redshift::AwsRedshiftSnapshotCopyGrantConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftSnapshotScheduleConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftUsageLimitConverter::new(Arc::clone(
        &injectable.redshift,
    )));
    injector.register(redshift::AwsRedshiftDataShareAuthorizationConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(
        redshift::AwsRedshiftDataShareConsumerAssociationConverter::new(Arc::clone(
            &injectable.redshift,
        )),
    );
    injector.register(redshift::AwsRedshiftEndpointAccessConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftEndpointAuthorizationConverter::new(
        Arc::clone(&injectable.redshift),
    ));
    injector.register(redshift::AwsRedshiftIntegrationConverter::new(Arc::clone(
        &injectable.redshift,
    )));
    injector.register(
        redshift::AwsRedshiftSnapshotScheduleAssociationConverter::new(Arc::clone(
            &injectable.redshift,
        )),
    );
    injector.register(rekognition::AwsRekognitionCollectionConverter::new(
        Arc::clone(&injectable.rekognition),
    ));
    injector.register(
        resiliencehub::AwsResilienceHubResiliencyPolicyConverter::new(Arc::clone(
            &injectable.resiliencehub,
        )),
    );
    injector.register(resourcegroups::AwsResourcegroupsGroupConverter::new(
        Arc::clone(&injectable.resourcegroups),
    ));
    injector.register(route53::AwsRoute53ZoneConverter::new(Arc::clone(
        &injectable.route53,
    )));
    injector.register(route53::AwsRoute53RecordConverter::new(Arc::clone(
        &injectable.route53,
    )));
    injector.register(route53::AwsRoute53CidrCollectionConverter::new(Arc::clone(
        &injectable.route53,
    )));
    injector.register(route53::AwsRoute53CidrLocationConverter::new(Arc::clone(
        &injectable.route53,
    )));
    injector.register(route53::AwsRoute53DelegationSetConverter::new(Arc::clone(
        &injectable.route53,
    )));
    injector.register(route53::AwsRoute53HealthCheckConverter::new(Arc::clone(
        &injectable.route53,
    )));
    injector.register(route53::AwsRoute53HostedZoneDnssecConverter::new(
        Arc::clone(&injectable.route53),
    ));
    injector.register(route53::AwsRoute53KeySigningKeyConverter::new(Arc::clone(
        &injectable.route53,
    )));
    injector.register(route53::AwsRoute53QueryLogConverter::new(Arc::clone(
        &injectable.route53,
    )));
    injector.register(route53::AwsRoute53TrafficPolicyConverter::new(Arc::clone(
        &injectable.route53,
    )));
    injector.register(route53::AwsRoute53TrafficPolicyInstanceConverter::new(
        Arc::clone(&injectable.route53),
    ));
    injector.register(
        route53::AwsRoute53VpcAssociationAuthorizationConverter::new(Arc::clone(
            &injectable.route53,
        )),
    );
    injector.register(route53::AwsRoute53ZoneAssociationConverter::new(
        Arc::clone(&injectable.route53),
    ));
    injector.register(
        route53domains::AwsRoute53DomainsRegisteredDomainConverter::new(Arc::clone(
            &injectable.route53domains,
        )),
    );
    injector.register(route53resolver::AwsRoute53ResolverEndpointConverter::new(
        Arc::clone(&injectable.route53resolver),
    ));
    injector.register(route53resolver::AwsRoute53ResolverRuleConverter::new(
        Arc::clone(&injectable.route53resolver),
    ));
    injector.register(route53resolver::AwsRoute53ResolverConfigConverter::new(
        Arc::clone(&injectable.route53resolver),
    ));
    injector.register(
        route53resolver::AwsRoute53ResolverDnssecConfigConverter::new(Arc::clone(
            &injectable.route53resolver,
        )),
    );
    injector.register(
        route53resolver::AwsRoute53ResolverFirewallConfigConverter::new(Arc::clone(
            &injectable.route53resolver,
        )),
    );
    injector.register(
        route53resolver::AwsRoute53ResolverFirewallDomainListConverter::new(Arc::clone(
            &injectable.route53resolver,
        )),
    );
    injector.register(
        route53resolver::AwsRoute53ResolverFirewallRuleConverter::new(Arc::clone(
            &injectable.route53resolver,
        )),
    );
    injector.register(
        route53resolver::AwsRoute53ResolverFirewallRuleGroupConverter::new(Arc::clone(
            &injectable.route53resolver,
        )),
    );
    injector.register(
        route53resolver::AwsRoute53ResolverFirewallRuleGroupAssociationConverter::new(Arc::clone(
            &injectable.route53resolver,
        )),
    );
    injector.register(
        route53resolver::AwsRoute53ResolverQueryLogConfigConverter::new(Arc::clone(
            &injectable.route53resolver,
        )),
    );
    injector.register(
        route53resolver::AwsRoute53ResolverQueryLogConfigAssociationConverter::new(Arc::clone(
            &injectable.route53resolver,
        )),
    );
    injector.register(
        route53resolver::AwsRoute53ResolverRuleAssociationConverter::new(Arc::clone(
            &injectable.route53resolver,
        )),
    );
    {
        let svc = Arc::clone(&injectable.s3);
        injector.register_with_scopes(
            s3::AwsS3BucketConverter::new(Arc::clone(&injectable.s3)),
            move || svc.scopes_with_state(),
        );
    }
    // S3 bucket sub-resources (single Option<String> fields)
    injector.register(s3::AwsS3BucketAccelerateConfigurationConverter::new(
        Arc::clone(&injectable.s3),
    ));
    injector.register(s3::AwsS3BucketAclConverter::new(Arc::clone(&injectable.s3)));
    injector.register(s3::AwsS3BucketCorsConfigurationConverter::new(Arc::clone(
        &injectable.s3,
    )));
    injector.register(s3::AwsS3BucketLifecycleConfigurationConverter::new(
        Arc::clone(&injectable.s3),
    ));
    injector.register(s3::AwsS3BucketLoggingConverter::new(Arc::clone(
        &injectable.s3,
    )));
    injector.register(s3::AwsS3BucketNotificationConverter::new(Arc::clone(
        &injectable.s3,
    )));
    injector.register(s3::AwsS3BucketObjectLockConfigurationConverter::new(
        Arc::clone(&injectable.s3),
    ));
    injector.register(s3::AwsS3BucketOwnershipControlsConverter::new(Arc::clone(
        &injectable.s3,
    )));
    injector.register(s3::AwsS3BucketPolicyConverter::new(Arc::clone(
        &injectable.s3,
    )));
    injector.register(s3::AwsS3BucketReplicationConfigurationConverter::new(
        Arc::clone(&injectable.s3),
    ));
    injector.register(s3::AwsS3BucketRequestPaymentConfigurationConverter::new(
        Arc::clone(&injectable.s3),
    ));
    injector.register(
        s3::AwsS3BucketServerSideEncryptionConfigurationConverter::new(Arc::clone(&injectable.s3)),
    );
    injector.register(s3::AwsS3BucketVersioningConverter::new(Arc::clone(
        &injectable.s3,
    )));
    injector.register(s3::AwsS3BucketWebsiteConfigurationConverter::new(
        Arc::clone(&injectable.s3),
    ));
    injector.register(s3::AwsS3BucketPublicAccessBlockConverter::new(Arc::clone(
        &injectable.s3,
    )));
    // S3 multi-entry configuration HashMaps
    injector.register(s3::AwsS3BucketAnalyticsConfigurationConverter::new(
        Arc::clone(&injectable.s3),
    ));
    injector.register(
        s3::AwsS3BucketIntelligentTieringConfigurationConverter::new(Arc::clone(&injectable.s3)),
    );
    injector.register(s3::AwsS3BucketMetricConverter::new(Arc::clone(
        &injectable.s3,
    )));
    // S3 object resources
    injector.register(s3::AwsS3ObjectConverter::new(Arc::clone(&injectable.s3)));
    injector.register(s3::AwsS3BucketObjectConverter::new(Arc::clone(
        &injectable.s3,
    )));
    injector.register(s3::AwsS3ObjectCopyConverter::new(Arc::clone(
        &injectable.s3,
    )));
    // S3 directory bucket
    injector.register(s3::AwsS3DirectoryBucketConverter::new(Arc::clone(
        &injectable.s3,
    )));
    injector.register(s3control::AwsS3controlAccessPointConverter::new(
        Arc::clone(&injectable.s3control),
    ));
    injector.register(s3control::AwsS3controlBucketConverter::new(Arc::clone(
        &injectable.s3control,
    )));
    injector.register(s3control::AwsS3controlAccessGrantConverter::new(
        Arc::clone(&injectable.s3control),
    ));
    injector.register(s3control::AwsS3controlAccessGrantsInstanceConverter::new(
        Arc::clone(&injectable.s3control),
    ));
    injector.register(
        s3control::AwsS3controlAccessGrantsInstanceResourcePolicyConverter::new(Arc::clone(
            &injectable.s3control,
        )),
    );
    injector.register(s3control::AwsS3controlAccessGrantsLocationConverter::new(
        Arc::clone(&injectable.s3control),
    ));
    injector.register(s3control::AwsS3controlAccessPointPolicyConverter::new(
        Arc::clone(&injectable.s3control),
    ));
    injector.register(
        s3control::AwsS3controlBucketLifecycleConfigurationConverter::new(Arc::clone(
            &injectable.s3control,
        )),
    );
    injector.register(s3control::AwsS3controlBucketPolicyConverter::new(
        Arc::clone(&injectable.s3control),
    ));
    injector.register(
        s3control::AwsS3controlDirectoryBucketAccessPointScopeConverter::new(Arc::clone(
            &injectable.s3control,
        )),
    );
    injector.register(s3control::AwsS3controlMultiRegionAccessPointConverter::new(
        Arc::clone(&injectable.s3control),
    ));
    injector.register(
        s3control::AwsS3controlMultiRegionAccessPointPolicyConverter::new(Arc::clone(
            &injectable.s3control,
        )),
    );
    injector.register(
        s3control::AwsS3controlObjectLambdaAccessPointConverter::new(Arc::clone(
            &injectable.s3control,
        )),
    );
    injector.register(
        s3control::AwsS3controlObjectLambdaAccessPointPolicyConverter::new(Arc::clone(
            &injectable.s3control,
        )),
    );
    injector.register(
        s3control::AwsS3controlStorageLensConfigurationConverter::new(Arc::clone(
            &injectable.s3control,
        )),
    );
    injector.register(s3tables::AwsS3tablesTableBucketConverter::new(Arc::clone(
        &injectable.s3tables,
    )));
    injector.register(s3tables::AwsS3tablesNamespaceConverter::new(Arc::clone(
        &injectable.s3tables,
    )));
    injector.register(sagemaker::AwsSagemakerDomainConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(sagemaker::AwsSagemakerEndpointConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(sagemaker::AwsSagemakerEndpointConfigurationConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerModelConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(sagemaker::AwsSagemakerNotebookInstanceConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerAppConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(
        sagemaker::AwsSagemakerDataQualityJobDefinitionConverter::new(Arc::clone(
            &injectable.sagemaker,
        )),
    );
    injector.register(sagemaker::AwsSagemakerFeatureGroupConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerModelPackageGroupConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(
        sagemaker::AwsSagemakerNotebookInstanceLifecycleConfigurationConverter::new(Arc::clone(
            &injectable.sagemaker,
        )),
    );
    injector.register(sagemaker::AwsSagemakerPipelineConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(sagemaker::AwsSagemakerSpaceConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(sagemaker::AwsSagemakerUserProfileConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerAppImageConfigConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerCodeRepositoryConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerDeviceConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(sagemaker::AwsSagemakerDeviceFleetConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerFlowDefinitionConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerHubConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(sagemaker::AwsSagemakerHumanTaskUiConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerImageConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(sagemaker::AwsSagemakerImageVersionConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerMlflowTrackingServerConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(
        sagemaker::AwsSagemakerModelPackageGroupPolicyConverter::new(Arc::clone(
            &injectable.sagemaker,
        )),
    );
    injector.register(sagemaker::AwsSagemakerMonitoringScheduleConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerProjectConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(
        sagemaker::AwsSagemakerServicecatalogPortfolioStatusConverter::new(Arc::clone(
            &injectable.sagemaker,
        )),
    );
    injector.register(sagemaker::AwsSagemakerStudioLifecycleConfigConverter::new(
        Arc::clone(&injectable.sagemaker),
    ));
    injector.register(sagemaker::AwsSagemakerWorkforceConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(sagemaker::AwsSagemakerWorkteamConverter::new(Arc::clone(
        &injectable.sagemaker,
    )));
    injector.register(scheduler::AwsSchedulerScheduleGroupConverter::new(
        Arc::clone(&injectable.scheduler),
    ));
    injector.register(scheduler::AwsSchedulerScheduleConverter::new(Arc::clone(
        &injectable.scheduler,
    )));
    injector.register(secretsmanager::AwsSecretsmanagerSecretConverter::new(
        Arc::clone(&injectable.secretsmanager),
    ));
    injector.register(
        secretsmanager::AwsSecretsmanagerSecretVersionConverter::new(Arc::clone(
            &injectable.secretsmanager,
        )),
    );
    injector.register(securityhub::AwsSecurityhubAccountConverter::new(
        Arc::clone(&injectable.securityhub),
    ));
    injector.register(
        securityhub::AwsSecurityhubStandardsSubscriptionConverter::new(Arc::clone(
            &injectable.securityhub,
        )),
    );
    injector.register(securityhub::AwsSecurityhubActionTargetConverter::new(
        Arc::clone(&injectable.securityhub),
    ));
    injector.register(securityhub::AwsSecurityhubAutomationRuleConverter::new(
        Arc::clone(&injectable.securityhub),
    ));
    injector.register(
        securityhub::AwsSecurityhubConfigurationPolicyConverter::new(Arc::clone(
            &injectable.securityhub,
        )),
    );
    injector.register(
        securityhub::AwsSecurityhubConfigurationPolicyAssociationConverter::new(Arc::clone(
            &injectable.securityhub,
        )),
    );
    injector.register(securityhub::AwsSecurityhubFindingAggregatorConverter::new(
        Arc::clone(&injectable.securityhub),
    ));
    injector.register(securityhub::AwsSecurityhubInsightConverter::new(
        Arc::clone(&injectable.securityhub),
    ));
    injector.register(securityhub::AwsSecurityhubInviteAccepterConverter::new(
        Arc::clone(&injectable.securityhub),
    ));
    injector.register(securityhub::AwsSecurityhubMemberConverter::new(Arc::clone(
        &injectable.securityhub,
    )));
    injector.register(
        securityhub::AwsSecurityhubOrganizationAdminAccountConverter::new(Arc::clone(
            &injectable.securityhub,
        )),
    );
    injector.register(
        securityhub::AwsSecurityhubOrganizationConfigurationConverter::new(Arc::clone(
            &injectable.securityhub,
        )),
    );
    injector.register(
        securityhub::AwsSecurityhubProductSubscriptionConverter::new(Arc::clone(
            &injectable.securityhub,
        )),
    );
    injector.register(securityhub::AwsSecurityhubStandardsControlConverter::new(
        Arc::clone(&injectable.securityhub),
    ));
    injector.register(
        securityhub::AwsSecurityhubStandardsControlAssociationConverter::new(Arc::clone(
            &injectable.securityhub,
        )),
    );
    injector.register(
        servicecatalog::AwsServicecatalogBudgetResourceAssociationConverter::new(Arc::clone(
            &injectable.servicecatalog,
        )),
    );
    injector.register(servicecatalog::AwsServicecatalogConstraintConverter::new(
        Arc::clone(&injectable.servicecatalog),
    ));
    injector.register(
        servicecatalog::AwsServicecatalogOrganizationsAccessConverter::new(Arc::clone(
            &injectable.servicecatalog,
        )),
    );
    injector.register(
        servicecatalog::AwsServicecatalogPortfolioShareConverter::new(Arc::clone(
            &injectable.servicecatalog,
        )),
    );
    injector.register(
        servicecatalog::AwsServicecatalogPrincipalPortfolioAssociationConverter::new(Arc::clone(
            &injectable.servicecatalog,
        )),
    );
    injector.register(
        servicecatalog::AwsServicecatalogProductPortfolioAssociationConverter::new(Arc::clone(
            &injectable.servicecatalog,
        )),
    );
    injector.register(
        servicecatalog::AwsServicecatalogProvisionedProductConverter::new(Arc::clone(
            &injectable.servicecatalog,
        )),
    );
    injector.register(
        servicecatalog::AwsServicecatalogProvisioningArtifactConverter::new(Arc::clone(
            &injectable.servicecatalog,
        )),
    );
    injector.register(
        servicecatalog::AwsServicecatalogServiceActionConverter::new(Arc::clone(
            &injectable.servicecatalog,
        )),
    );
    injector.register(servicecatalog::AwsServicecatalogTagOptionConverter::new(
        Arc::clone(&injectable.servicecatalog),
    ));
    injector.register(
        servicecatalog::AwsServicecatalogTagOptionResourceAssociationConverter::new(Arc::clone(
            &injectable.servicecatalog,
        )),
    );
    injector.register(
        servicecatalogappregistry::AwsServicecatalogappregistryApplicationConverter::new(
            Arc::clone(&injectable.servicecatalogappregistry),
        ),
    );
    injector.register(
        servicediscovery::AwsServiceDiscoveryPrivateDnsNamespaceConverter::new(Arc::clone(
            &injectable.servicediscovery,
        )),
    );
    injector.register(servicediscovery::AwsServiceDiscoveryServiceConverter::new(
        Arc::clone(&injectable.servicediscovery),
    ));
    injector.register(servicequotas::AwsServicequotasServiceQuotaConverter::new(
        Arc::clone(&injectable.servicequotas),
    ));
    injector.register(ses::AwsSesv2EmailIdentityConverter::new(Arc::clone(
        &injectable.ses,
    )));
    injector.register(ses::AwsSesv2ConfigurationSetConverter::new(Arc::clone(
        &injectable.ses,
    )));
    injector.register(ses::AwsSesv2DedicatedIpPoolConverter::new(Arc::clone(
        &injectable.ses,
    )));
    injector.register(ses::AwsSesv2AccountSuppressionAttributesConverter::new(
        Arc::clone(&injectable.ses),
    ));
    injector.register(ses::AwsSesv2AccountVdmAttributesConverter::new(Arc::clone(
        &injectable.ses,
    )));
    injector.register(ses::AwsSesv2ConfigurationSetEventDestinationConverter::new(
        Arc::clone(&injectable.ses),
    ));
    injector.register(ses::AwsSesv2ContactListConverter::new(Arc::clone(
        &injectable.ses,
    )));
    injector.register(ses::AwsSesv2DedicatedIpAssignmentConverter::new(
        Arc::clone(&injectable.ses),
    ));
    injector.register(ses::AwsSesv2EmailIdentityFeedbackAttributesConverter::new(
        Arc::clone(&injectable.ses),
    ));
    injector.register(ses::AwsSesv2EmailIdentityMailFromAttributesConverter::new(
        Arc::clone(&injectable.ses),
    ));
    injector.register(ses::AwsSesv2EmailIdentityPolicyConverter::new(Arc::clone(
        &injectable.ses,
    )));
    injector.register(sesv1::AwsSesEmailIdentityConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesDomainIdentityConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesActiveReceiptRuleSetConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesConfigurationSetConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesDomainDkimConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesDomainIdentityVerificationConverter::new(
        Arc::clone(&injectable.sesv1),
    ));
    injector.register(sesv1::AwsSesDomainMailFromConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesEventDestinationConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesIdentityNotificationTopicConverter::new(
        Arc::clone(&injectable.sesv1),
    ));
    injector.register(sesv1::AwsSesIdentityPolicyConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesReceiptFilterConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesReceiptRuleConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesReceiptRuleSetConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(sesv1::AwsSesTemplateConverter::new(Arc::clone(
        &injectable.sesv1,
    )));
    injector.register(shield::AwsShieldProtectionConverter::new(Arc::clone(
        &injectable.shield,
    )));
    injector.register(
        shield::AwsShieldApplicationLayerAutomaticResponseConverter::new(Arc::clone(
            &injectable.shield,
        )),
    );
    injector.register(
        shield::AwsShieldDrtAccessLogBucketAssociationConverter::new(Arc::clone(
            &injectable.shield,
        )),
    );
    injector.register(shield::AwsShieldDrtAccessRoleArnAssociationConverter::new(
        Arc::clone(&injectable.shield),
    ));
    injector.register(shield::AwsShieldProactiveEngagementConverter::new(
        Arc::clone(&injectable.shield),
    ));
    injector.register(shield::AwsShieldProtectionGroupConverter::new(Arc::clone(
        &injectable.shield,
    )));
    injector.register(
        shield::AwsShieldProtectionHealthCheckAssociationConverter::new(Arc::clone(
            &injectable.shield,
        )),
    );
    injector.register(shield::AwsShieldSubscriptionConverter::new(Arc::clone(
        &injectable.shield,
    )));
    injector.register(signer::AwsSignerSigningProfileConverter::new(Arc::clone(
        &injectable.signer,
    )));
    injector.register(simpledbv2::AwsSimpleDbDomainConverter::new(Arc::clone(
        &injectable.simpledbv2,
    )));
    {
        let sns_for_scopes = Arc::clone(&injectable.sns);
        injector.register_with_scopes(
            sns::AwsSnsTopicConverter::new(Arc::clone(&injectable.sns)),
            move || sns_for_scopes.scopes_with_state(),
        );
    }
    {
        let sns_for_scopes = Arc::clone(&injectable.sns);
        injector.register_with_scopes(
            sns::AwsSnsTopicSubscriptionConverter::new(Arc::clone(&injectable.sns)),
            move || sns_for_scopes.scopes_with_state(),
        );
    }
    {
        let sqs_for_scopes = Arc::clone(&injectable.sqs);
        injector.register_with_scopes(
            sqs::AwsSqsQueueConverter::new(Arc::clone(&injectable.sqs)),
            move || sqs_for_scopes.scopes_with_state(),
        );
    }
    injector.register(ssm::AwsSsmParameterConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssm::AwsSsmActivationConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssm::AwsSsmAssociationConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssm::AwsSsmDefaultPatchBaselineConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssm::AwsSsmDocumentConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssm::AwsSsmMaintenanceWindowConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssm::AwsSsmMaintenanceWindowTargetConverter::new(
        Arc::clone(&injectable.ssm),
    ));
    injector.register(ssm::AwsSsmMaintenanceWindowTaskConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssm::AwsSsmPatchBaselineConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssm::AwsSsmPatchGroupConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssm::AwsSsmResourceDataSyncConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssm::AwsSsmServiceSettingConverter::new(Arc::clone(
        &injectable.ssm,
    )));
    injector.register(ssoadmin::AwsSsoadminPermissionSetConverter::new(
        Arc::clone(&injectable.ssoadmin),
    ));
    injector.register(ssoadmin::AwsSsoadminAccountAssignmentConverter::new(
        Arc::clone(&injectable.ssoadmin),
    ));
    injector.register(ssoadmin::AwsSsoadminApplicationAccessScopeConverter::new(
        Arc::clone(&injectable.ssoadmin),
    ));
    injector.register(
        ssoadmin::AwsSsoadminApplicationAssignmentConfigurationConverter::new(Arc::clone(
            &injectable.ssoadmin,
        )),
    );
    injector.register(ssoadmin::AwsSsoadminApplicationAssignmentConverter::new(
        Arc::clone(&injectable.ssoadmin),
    ));
    injector.register(ssoadmin::AwsSsoadminApplicationConverter::new(Arc::clone(
        &injectable.ssoadmin,
    )));
    injector.register(
        ssoadmin::AwsSsoadminCustomerManagedPolicyAttachmentConverter::new(Arc::clone(
            &injectable.ssoadmin,
        )),
    );
    injector.register(
        ssoadmin::AwsSsoadminInstanceAccessControlAttributesConverter::new(Arc::clone(
            &injectable.ssoadmin,
        )),
    );
    injector.register(ssoadmin::AwsSsoadminManagedPolicyAttachmentConverter::new(
        Arc::clone(&injectable.ssoadmin),
    ));
    injector.register(
        ssoadmin::AwsSsoadminPermissionSetInlinePolicyConverter::new(Arc::clone(
            &injectable.ssoadmin,
        )),
    );
    injector.register(
        ssoadmin::AwsSsoadminPermissionsBoundaryAttachmentConverter::new(Arc::clone(
            &injectable.ssoadmin,
        )),
    );
    injector.register(ssoadmin::AwsSsoadminTrustedTokenIssuerConverter::new(
        Arc::clone(&injectable.ssoadmin),
    ));
    injector.register(stepfunctions::AwsSfnStateMachineConverter::new(Arc::clone(
        &injectable.stepfunctions,
    )));
    injector.register(swf::AwsSwfDomainConverter::new(Arc::clone(&injectable.swf)));
    injector.register(synthetics::AwsSyntheticsCanaryConverter::new(Arc::clone(
        &injectable.synthetics,
    )));
    injector.register(
        timestreaminfluxdb::AwsTimestreaminfluxdbDbInstanceConverter::new(Arc::clone(
            &injectable.timestreaminfluxdb,
        )),
    );
    injector.register(
        timestreamquery::AwsTimestreamQueryScheduledQueryConverter::new(Arc::clone(
            &injectable.timestreamquery,
        )),
    );
    injector.register(timestreamwrite::AwsTimestreamwriteDatabaseConverter::new(
        Arc::clone(&injectable.timestreamwrite),
    ));
    injector.register(timestreamwrite::AwsTimestreamwriteTableConverter::new(
        Arc::clone(&injectable.timestreamwrite),
    ));
    injector.register(transcribe::AwsTranscribeVocabularyConverter::new(
        Arc::clone(&injectable.transcribe),
    ));
    injector.register(transcribe::AwsTranscribeLanguageModelConverter::new(
        Arc::clone(&injectable.transcribe),
    ));
    injector.register(transfer::AwsTransferServerConverter::new(Arc::clone(
        &injectable.transfer,
    )));
    injector.register(transfer::AwsTransferUserConverter::new(Arc::clone(
        &injectable.transfer,
    )));
    injector.register(transfer::AwsTransferAccessConverter::new(Arc::clone(
        &injectable.transfer,
    )));
    injector.register(transfer::AwsTransferAgreementConverter::new(Arc::clone(
        &injectable.transfer,
    )));
    injector.register(transfer::AwsTransferCertificateConverter::new(Arc::clone(
        &injectable.transfer,
    )));
    injector.register(transfer::AwsTransferConnectorConverter::new(Arc::clone(
        &injectable.transfer,
    )));
    injector.register(transfer::AwsTransferProfileConverter::new(Arc::clone(
        &injectable.transfer,
    )));
    injector.register(transfer::AwsTransferSshKeyConverter::new(Arc::clone(
        &injectable.transfer,
    )));
    injector.register(transfer::AwsTransferTagConverter::new(Arc::clone(
        &injectable.transfer,
    )));
    injector.register(transfer::AwsTransferWorkflowConverter::new(Arc::clone(
        &injectable.transfer,
    )));
    injector.register(vpclattice::AwsVpcLatticeServiceConverter::new(Arc::clone(
        &injectable.vpclattice,
    )));
    injector.register(vpclattice::AwsVpcLatticeServiceNetworkConverter::new(
        Arc::clone(&injectable.vpclattice),
    ));
    injector.register(vpclattice::AwsVpcLatticeTargetGroupConverter::new(
        Arc::clone(&injectable.vpclattice),
    ));
    injector.register(vpclattice::AwsVpcLatticeListenerConverter::new(Arc::clone(
        &injectable.vpclattice,
    )));
    injector.register(
        vpclattice::AwsVpcLatticeAccessLogSubscriptionConverter::new(Arc::clone(
            &injectable.vpclattice,
        )),
    );
    injector.register(vpclattice::AwsVpcLatticeAuthPolicyConverter::new(
        Arc::clone(&injectable.vpclattice),
    ));
    injector.register(vpclattice::AwsVpcLatticeListenerRuleConverter::new(
        Arc::clone(&injectable.vpclattice),
    ));
    injector.register(
        vpclattice::AwsVpcLatticeResourceConfigurationConverter::new(Arc::clone(
            &injectable.vpclattice,
        )),
    );
    injector.register(vpclattice::AwsVpcLatticeResourceGatewayConverter::new(
        Arc::clone(&injectable.vpclattice),
    ));
    injector.register(vpclattice::AwsVpcLatticeResourcePolicyConverter::new(
        Arc::clone(&injectable.vpclattice),
    ));
    injector.register(
        vpclattice::AwsVpcLatticeServiceNetworkResourceAssociationConverter::new(Arc::clone(
            &injectable.vpclattice,
        )),
    );
    injector.register(
        vpclattice::AwsVpcLatticeServiceNetworkServiceAssociationConverter::new(Arc::clone(
            &injectable.vpclattice,
        )),
    );
    injector.register(
        vpclattice::AwsVpcLatticeServiceNetworkVpcAssociationConverter::new(Arc::clone(
            &injectable.vpclattice,
        )),
    );
    injector.register(
        vpclattice::AwsVpcLatticeTargetGroupAttachmentConverter::new(Arc::clone(
            &injectable.vpclattice,
        )),
    );
    injector.register(wafv2::AwsWafv2WebAclConverter::new(Arc::clone(
        &injectable.wafv2,
    )));
    injector.register(wafv2::AwsWafv2IpSetConverter::new(Arc::clone(
        &injectable.wafv2,
    )));
    injector.register(wafv2::AwsWafv2RuleGroupConverter::new(Arc::clone(
        &injectable.wafv2,
    )));
    injector.register(workspaces::AwsWorkspacesWorkspaceConverter::new(
        Arc::clone(&injectable.workspaces),
    ));
    injector.register(workspaces::AwsWorkspacesDirectoryConverter::new(
        Arc::clone(&injectable.workspaces),
    ));
    injector.register(xray::AwsXrayGroupConverter::new(Arc::clone(
        &injectable.xray,
    )));
    injector.register(xray::AwsXraySamplingRuleConverter::new(Arc::clone(
        &injectable.xray,
    )));

    let ctx = winterbaume_terraform::ConversionContext {
        default_account_id: account_id.to_string(),
        default_region: default_region.to_string(),
    };

    let report = injector.inject_all(&tfstate, &ctx).await;
    Ok(report)
}

// ---------------------------------------------------------------------------
// CLI / env / config parsing
// ---------------------------------------------------------------------------

/// CLI flag set, parsed by `clap`. Each field also reads from its `WB_*` env
/// variable when the flag is omitted, so `clap` resolves "CLI > env" on its
/// own; the config-file and built-in-default fallbacks are layered on top in
/// [`parse_options`].
#[derive(Parser, Debug)]
#[command(
    name = "winterbaume-server",
    about = "Standalone HTTP server for winterbaume mock AWS services",
    long_about = "Configuration precedence: CLI flags > environment variables > \
                  config file > defaults",
    disable_help_flag = true
)]
struct CliArgs {
    /// Print help and exit. (`-h` is reserved for `--host` for backwards
    /// compatibility with the previous hand-written parser.)
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Bind address.
    #[arg(long, short = 'h', value_name = "HOST", env = "WB_HOST")]
    host: Option<String>,

    /// Bind port.
    #[arg(long, short = 'p', value_name = "PORT", env = "WB_PORT")]
    port: Option<u16>,

    /// AWS account ID for state injection.
    #[arg(long, value_name = "ID", env = "WB_ACCOUNT_ID")]
    account_id: Option<String>,

    /// Default AWS region.
    #[arg(long, value_name = "REGION", env = "WB_REGION")]
    region: Option<String>,

    /// Load a Terraform state file at startup.
    #[arg(long, value_name = "FILE", env = "WB_TFSTATE")]
    tfstate: Option<String>,

    /// Load a TOML configuration file.
    #[arg(long, value_name = "FILE", env = "WB_CONFIG")]
    config: Option<String>,

    /// Redis URL for SQS backend (requires `backend-sqs-redis` feature).
    #[arg(long, value_name = "URL", env = "WB_SQS_BACKEND")]
    sqs_backend: Option<String>,

    /// Redis URL for DynamoDB backend (requires `backend-dynamodb-redis`
    /// feature).
    #[arg(long, value_name = "URL", env = "WB_DYNAMODB_BACKEND")]
    dynamodb_backend: Option<String>,

    /// Filesystem directory for S3/Glacier/EBS blob storage (requires
    /// `backend-vfs-fs` feature).
    #[arg(long, value_name = "DIR", env = "WB_VFS_DIR")]
    vfs_dir: Option<String>,

    /// DuckDB path (or `:memory:`) for Athena / Redshift Data (requires
    /// `backend-sqlengine-duckdb` or `backend-sqlengine-duckdb-bundled`
    /// feature).
    #[arg(long, value_name = "PATH", env = "WB_SQLENGINE_DUCKDB")]
    sqlengine_duckdb: Option<String>,
}

/// Builds the "Compiled-in backend features" block shown after `--help`.
/// Done at runtime so the `cfg!` macros stay readable.
fn compiled_features_summary() -> String {
    let mut out = String::from("Compiled-in backend features:\n");
    let mark = |enabled: bool| if enabled { "[x]" } else { "[ ]" };

    out.push_str(&format!(
        "  {} backend-sqs-redis\n",
        mark(cfg!(feature = "backend-sqs-redis"))
    ));
    out.push_str(&format!(
        "  {} backend-dynamodb-redis\n",
        mark(cfg!(feature = "backend-dynamodb-redis"))
    ));
    out.push_str(&format!(
        "  {} backend-vfs-fs\n",
        mark(cfg!(feature = "backend-vfs-fs"))
    ));

    if cfg!(feature = "backend-sqlengine-duckdb-bundled") {
        out.push_str("  [x] backend-sqlengine-duckdb-bundled (DuckDB compiled in)\n");
    } else if cfg!(feature = "backend-sqlengine-duckdb") {
        out.push_str("  [x] backend-sqlengine-duckdb (linked to prebuilt libduckdb)\n");
    } else {
        out.push_str("  [ ] backend-sqlengine-duckdb\n");
    }

    out
}

/// Parse all CLI args, env vars, and optional config file into a single
/// [`ServerOptions`] struct using the precedence:
///   CLI flags > env vars > config file > built-in defaults.
fn parse_options() -> ServerOptions {
    let cmd = CliArgs::command().after_help(compiled_features_summary());
    let matches = cmd.get_matches();
    let cli = CliArgs::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());

    // --- Config file --------------------------------------------------------

    let file_cfg: ConfigFile = match cli.config.as_deref() {
        None => ConfigFile::default(),
        Some(path) => {
            let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
                eprintln!("error: cannot read config file '{}': {}", path, e);
                std::process::exit(1);
            });
            toml::from_str(&content).unwrap_or_else(|e| {
                eprintln!("error: invalid config file '{}': {}", path, e);
                std::process::exit(1);
            })
        }
    };

    let file_backends = file_cfg.backends.unwrap_or_default();

    // --- Merge: (CLI > env, resolved by clap) > config > default ------------

    let host = cli
        .host
        .or(file_cfg.host)
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let port = cli.port.or(file_cfg.port).unwrap_or(5555);

    let account_id = cli
        .account_id
        .or(file_cfg.account_id)
        .unwrap_or_else(|| winterbaume_core::DEFAULT_ACCOUNT_ID.to_string());

    let default_region = cli
        .region
        .or(file_cfg.region)
        .unwrap_or_else(|| "us-east-1".to_string());

    let tfstate_path = cli.tfstate.or(file_cfg.tfstate);

    let sqs_backend = cli.sqs_backend.or(file_backends.sqs);
    let dynamodb_backend = cli.dynamodb_backend.or(file_backends.dynamodb);
    let vfs_dir = cli.vfs_dir.or(file_backends.vfs_dir);
    let sqlengine_duckdb = cli.sqlengine_duckdb.or(file_backends.sqlengine_duckdb);

    ServerOptions {
        host,
        port,
        account_id,
        default_region,
        tfstate_path,
        backends: BackendOptions {
            sqs_backend,
            dynamodb_backend,
            vfs_dir,
            sqlengine_duckdb,
        },
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let opts = parse_options();

    let addr: SocketAddr = format!("{}:{}", opts.host, opts.port).parse()?;
    let listener = TcpListener::bind(addr).await?;

    let (services, injectable) = register_all_services(&opts.backends).await?;

    // Load terraform state if provided
    if let Some(ref path) = opts.tfstate_path {
        tracing::info!("Loading Terraform state from: {}", path);
        match load_tfstate(path, &injectable, &opts.account_id, &opts.default_region).await {
            Ok(report) => {
                tracing::info!(
                    "Terraform state loaded: {} resources injected, {} types skipped",
                    report.injected,
                    report.skipped.len()
                );
                if !report.skipped.is_empty() {
                    tracing::debug!("Skipped resource types: {}", report.skipped.join(", "));
                }
                for warning in &report.warnings {
                    tracing::warn!("tfstate: {}", warning);
                }
                for error in &report.errors {
                    tracing::error!("tfstate: {}", error);
                }
                if report.has_errors() {
                    tracing::warn!(
                        "Terraform state had {} injection errors; server will start with partial state",
                        report.errors.len()
                    );
                }
            }
            Err(e) => {
                eprintln!("Failed to load Terraform state file '{}': {}", path, e);
                std::process::exit(1);
            }
        }
    }

    let router = Arc::new(Router::new(services));

    let service_names: Vec<&str> = router
        .routes
        .iter()
        .map(|r| r.service.service_name())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    tracing::info!("winterbaume server listening on {}", addr);
    tracing::info!("Registered services: {}", service_names.join(", "));
    tracing::info!("Configure AWS SDK endpoint: http://{}", addr);

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let router = Arc::clone(&router);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                let router = Arc::clone(&router);
                async move { handle_request(router, remote_addr, req).await }
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                tracing::debug!("Connection error from {}: {}", remote_addr, err);
            }
        });
    }
}
