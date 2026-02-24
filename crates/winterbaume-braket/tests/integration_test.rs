use std::sync::{Arc, Mutex};

use aws_sdk_braket::config::BehaviorVersion;
use aws_sdk_braket::types::{
    AlgorithmSpecification, ContainerImage, DeviceConfig, InstanceConfig, JobOutputDataConfig,
};
use winterbaume_braket::BraketService;
use winterbaume_core::{MockAws, StatefulService};

const SV1: &str = "arn:aws:braket:::device/quantum-simulator/amazon/sv1";

async fn make_client() -> aws_sdk_braket::Client {
    let mock = MockAws::builder()
        .with_service(BraketService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_braket::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_braket::Client::new(&config)
}

#[tokio::test]
async fn test_get_seeded_device() {
    let client = make_client().await;
    let resp = client
        .get_device()
        .device_arn(SV1)
        .send()
        .await
        .expect("get_device");
    assert_eq!(resp.device_arn(), SV1);
    assert_eq!(resp.device_name(), "SV1");
}

#[tokio::test]
async fn test_search_devices_lists_seeded() {
    let client = make_client().await;
    let resp = client
        .search_devices()
        .send()
        .await
        .expect("search_devices");
    assert_eq!(resp.devices().len(), 1);
    assert_eq!(resp.devices()[0].device_arn(), SV1);
}

#[tokio::test]
async fn test_get_unknown_device() {
    let client = make_client().await;
    let err = client
        .get_device()
        .device_arn("arn:aws:braket:::device/missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

fn job_essentials() -> (
    AlgorithmSpecification,
    DeviceConfig,
    InstanceConfig,
    JobOutputDataConfig,
) {
    let alg = AlgorithmSpecification::builder()
        .container_image(
            ContainerImage::builder()
                .uri("123456789012.dkr.ecr.us-east-1.amazonaws.com/braket-job:latest")
                .build()
                .expect("img"),
        )
        .build();
    let dev = DeviceConfig::builder().device(SV1).build().expect("dev");
    let inst = InstanceConfig::builder()
        .instance_type("ml.m5.large".into())
        .volume_size_in_gb(30)
        .build()
        .expect("inst");
    let out = JobOutputDataConfig::builder()
        .s3_path("s3://my-bucket/braket")
        .build()
        .expect("out");
    (alg, dev, inst, out)
}

#[tokio::test]
async fn test_create_get_cancel_job() {
    let client = make_client().await;
    let (alg, dev, inst, out) = job_essentials();
    let resp = client
        .create_job()
        .client_token("ct")
        .job_name("my-job")
        .role_arn("arn:aws:iam::123456789012:role/braket-role")
        .algorithm_specification(alg)
        .device_config(dev)
        .instance_config(inst)
        .output_data_config(out)
        .send()
        .await
        .expect("create_job");
    let arn = resp.job_arn().to_string();
    let got = client
        .get_job()
        .job_arn(&arn)
        .send()
        .await
        .expect("get_job");
    assert_eq!(got.status().as_str(), "QUEUED");

    let cancelled = client
        .cancel_job()
        .job_arn(&arn)
        .send()
        .await
        .expect("cancel_job");
    assert_eq!(cancelled.cancellation_status().as_str(), "CANCELLING");
}

#[tokio::test]
async fn test_create_get_cancel_quantum_task() {
    let client = make_client().await;
    let resp = client
        .create_quantum_task()
        .client_token("ct")
        .device_arn(SV1)
        .action("{\"braketSchemaHeader\":{}}")
        .shots(100)
        .output_s3_bucket("my-bucket")
        .output_s3_key_prefix("results/")
        .send()
        .await
        .expect("create_task");
    let arn = resp.quantum_task_arn().to_string();
    let got = client
        .get_quantum_task()
        .quantum_task_arn(&arn)
        .send()
        .await
        .expect("get_task");
    assert_eq!(got.device_arn(), SV1);
    let cancel = client
        .cancel_quantum_task()
        .quantum_task_arn(&arn)
        .client_token("ct")
        .send()
        .await
        .expect("cancel_task");
    assert_eq!(cancel.cancellation_status().as_str(), "CANCELLING");
}

#[tokio::test]
async fn test_search_jobs_and_tasks() {
    let client = make_client().await;
    let (alg, dev, inst, out) = job_essentials();
    client
        .create_job()
        .client_token("ct")
        .job_name("j")
        .role_arn("arn:aws:iam::123456789012:role/r")
        .algorithm_specification(alg)
        .device_config(dev)
        .instance_config(inst)
        .output_data_config(out)
        .send()
        .await
        .expect("create");
    let resp = client.search_jobs().send().await.expect("search_jobs");
    assert_eq!(resp.jobs().len(), 1);

    client
        .create_quantum_task()
        .client_token("ct")
        .device_arn(SV1)
        .action("{}")
        .shots(10)
        .output_s3_bucket("b")
        .output_s3_key_prefix("k")
        .send()
        .await
        .expect("create_task");
    let resp = client
        .search_quantum_tasks()
        .send()
        .await
        .expect("search_tasks");
    assert_eq!(resp.quantum_tasks().len(), 1);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = BraketService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .expect("restore");
    assert_eq!(events.lock().unwrap().len(), 1);
}

/// Regression: GET/POST/DELETE /tags/{resourceArn+} routes have ARN colons
/// percent-encoded by the SDK. The handler must URL-decode the trailing
/// segments before using them as the state-map key, otherwise tags written
/// for "arn:aws:braket:..." would never be found by list_tags_for_resource.
#[tokio::test]
async fn test_tag_resource_url_encoded_arn_roundtrip() {
    let client = make_client().await;
    let (alg, dev, inst, out) = job_essentials();
    let job = client
        .create_job()
        .client_token("ct")
        .job_name("tag-roundtrip-job")
        .role_arn("arn:aws:iam::123456789012:role/r")
        .algorithm_specification(alg)
        .device_config(dev)
        .instance_config(inst)
        .output_data_config(out)
        .send()
        .await
        .expect("create_job");
    let arn = job.job_arn().to_string();
    assert!(arn.contains(':'));

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .send()
        .await
        .expect("tag_resource");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource");
    let tags = resp.tags().expect("tags");
    assert_eq!(tags.get("env"), Some(&"prod".to_string()));

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource after untag");
    assert!(resp.tags().map(|t| t.is_empty()).unwrap_or(true));
}
