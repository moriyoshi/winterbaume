//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-timestream-influxdb

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDbClusterInput {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    pub db_instance_type: String,
    #[serde(rename = "dbParameterGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_identifier: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "failoverMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<String>,
    #[serde(rename = "logDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "vpcSecurityGroupIds")]
    #[serde(default)]
    pub vpc_security_group_ids: Vec<String>,
    #[serde(rename = "vpcSubnetIds")]
    #[serde(default)]
    pub vpc_subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogDeliveryConfiguration {
    #[serde(rename = "s3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3Configuration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Configuration {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceSchedule {
    #[serde(rename = "preferredMaintenanceWindow")]
    #[serde(default)]
    pub preferred_maintenance_window: String,
    #[serde(default)]
    pub timezone: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDbClusterOutput {
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_id: Option<String>,
    #[serde(rename = "dbClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDbInstanceInput {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    pub allocated_storage: i32,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    pub db_instance_type: String,
    #[serde(rename = "dbParameterGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_identifier: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "logDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "vpcSecurityGroupIds")]
    #[serde(default)]
    pub vpc_security_group_ids: Vec<String>,
    #[serde(rename = "vpcSubnetIds")]
    #[serde(default)]
    pub vpc_subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDbInstanceOutput {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_id: Option<String>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbParameterGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_identifier: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "influxAuthParametersSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influx_auth_parameters_secret_arn: Option<String>,
    #[serde(rename = "instanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_mode: Option<String>,
    #[serde(rename = "instanceModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_modes: Option<Vec<String>>,
    #[serde(rename = "lastMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_maintenance_time: Option<String>,
    #[serde(rename = "logDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "nextMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_maintenance_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "secondaryAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    #[serde(rename = "vpcSubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDbParameterGroupInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parameters {
    #[serde(rename = "InfluxDBv2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influx_d_bv2: Option<InfluxDBv2Parameters>,
    #[serde(rename = "InfluxDBv3Core")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influx_d_bv3_core: Option<InfluxDBv3CoreParameters>,
    #[serde(rename = "InfluxDBv3Enterprise")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influx_d_bv3_enterprise: Option<InfluxDBv3EnterpriseParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InfluxDBv2Parameters {
    #[serde(rename = "fluxLogEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flux_log_enabled: Option<bool>,
    #[serde(rename = "httpIdleTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_idle_timeout: Option<Duration>,
    #[serde(rename = "httpReadHeaderTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_read_header_timeout: Option<Duration>,
    #[serde(rename = "httpReadTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_read_timeout: Option<Duration>,
    #[serde(rename = "httpWriteTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_write_timeout: Option<Duration>,
    #[serde(rename = "influxqlMaxSelectBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influxql_max_select_buckets: Option<i64>,
    #[serde(rename = "influxqlMaxSelectPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influxql_max_select_point: Option<i64>,
    #[serde(rename = "influxqlMaxSelectSeries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influxql_max_select_series: Option<i64>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(rename = "metricsDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_disabled: Option<bool>,
    #[serde(rename = "noTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_tasks: Option<bool>,
    #[serde(rename = "pprofDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pprof_disabled: Option<bool>,
    #[serde(rename = "queryConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_concurrency: Option<i32>,
    #[serde(rename = "queryInitialMemoryBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_initial_memory_bytes: Option<i64>,
    #[serde(rename = "queryMaxMemoryBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_max_memory_bytes: Option<i64>,
    #[serde(rename = "queryMemoryBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_memory_bytes: Option<i64>,
    #[serde(rename = "queryQueueSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_queue_size: Option<i32>,
    #[serde(rename = "sessionLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_length: Option<i32>,
    #[serde(rename = "sessionRenewDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_renew_disabled: Option<bool>,
    #[serde(rename = "storageCacheMaxMemorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_cache_max_memory_size: Option<i64>,
    #[serde(rename = "storageCacheSnapshotMemorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_cache_snapshot_memory_size: Option<i64>,
    #[serde(rename = "storageCacheSnapshotWriteColdDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_cache_snapshot_write_cold_duration: Option<Duration>,
    #[serde(rename = "storageCompactFullWriteColdDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_compact_full_write_cold_duration: Option<Duration>,
    #[serde(rename = "storageCompactThroughputBurst")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_compact_throughput_burst: Option<i64>,
    #[serde(rename = "storageMaxConcurrentCompactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_max_concurrent_compactions: Option<i32>,
    #[serde(rename = "storageMaxIndexLogFileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_max_index_log_file_size: Option<i64>,
    #[serde(rename = "storageNoValidateFieldSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_no_validate_field_size: Option<bool>,
    #[serde(rename = "storageRetentionCheckInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_retention_check_interval: Option<Duration>,
    #[serde(rename = "storageSeriesFileMaxConcurrentSnapshotCompactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_series_file_max_concurrent_snapshot_compactions: Option<i32>,
    #[serde(rename = "storageSeriesIdSetCacheSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_series_id_set_cache_size: Option<i64>,
    #[serde(rename = "storageWalMaxConcurrentWrites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_wal_max_concurrent_writes: Option<i32>,
    #[serde(rename = "storageWalMaxWriteDelay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_wal_max_write_delay: Option<Duration>,
    #[serde(rename = "tracingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_type: Option<String>,
    #[serde(rename = "uiDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_disabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Duration {
    #[serde(rename = "durationType")]
    #[serde(default)]
    pub duration_type: String,
    #[serde(default)]
    pub value: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InfluxDBv3CoreParameters {
    #[serde(rename = "dataFusionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_config: Option<String>,
    #[serde(rename = "dataFusionMaxParquetFanout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_max_parquet_fanout: Option<i32>,
    #[serde(rename = "dataFusionNumThreads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_num_threads: Option<i32>,
    #[serde(rename = "dataFusionRuntimeDisableLifoSlot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_disable_lifo_slot: Option<bool>,
    #[serde(rename = "dataFusionRuntimeEventInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_event_interval: Option<i32>,
    #[serde(rename = "dataFusionRuntimeGlobalQueueInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_global_queue_interval: Option<i32>,
    #[serde(rename = "dataFusionRuntimeMaxBlockingThreads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_max_blocking_threads: Option<i32>,
    #[serde(rename = "dataFusionRuntimeMaxIoEventsPerTick")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_max_io_events_per_tick: Option<i32>,
    #[serde(rename = "dataFusionRuntimeThreadKeepAlive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_thread_keep_alive: Option<Duration>,
    #[serde(rename = "dataFusionRuntimeThreadPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_thread_priority: Option<i32>,
    #[serde(rename = "dataFusionRuntimeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_type: Option<String>,
    #[serde(rename = "dataFusionUseCachedParquetLoader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_use_cached_parquet_loader: Option<bool>,
    #[serde(rename = "deleteGracePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_grace_period: Option<Duration>,
    #[serde(rename = "disableParquetMemCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_parquet_mem_cache: Option<bool>,
    #[serde(rename = "distinctCacheEvictionInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_cache_eviction_interval: Option<Duration>,
    #[serde(rename = "execMemPoolBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_mem_pool_bytes: Option<PercentOrAbsoluteLong>,
    #[serde(rename = "forceSnapshotMemThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_snapshot_mem_threshold: Option<PercentOrAbsoluteLong>,
    #[serde(rename = "gen1Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gen1_duration: Option<Duration>,
    #[serde(rename = "gen1LookbackDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gen1_lookback_duration: Option<Duration>,
    #[serde(rename = "hardDeleteDefaultDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_delete_default_duration: Option<Duration>,
    #[serde(rename = "lastCacheEvictionInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_cache_eviction_interval: Option<Duration>,
    #[serde(rename = "logFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_filter: Option<String>,
    #[serde(rename = "logFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_format: Option<String>,
    #[serde(rename = "maxHttpRequestSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_http_request_size: Option<i64>,
    #[serde(rename = "parquetMemCachePruneInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_mem_cache_prune_interval: Option<Duration>,
    #[serde(rename = "parquetMemCachePrunePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_mem_cache_prune_percentage: Option<f32>,
    #[serde(rename = "parquetMemCacheQueryPathDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_mem_cache_query_path_duration: Option<Duration>,
    #[serde(rename = "parquetMemCacheSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_mem_cache_size: Option<PercentOrAbsoluteLong>,
    #[serde(rename = "preemptiveCacheAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemptive_cache_age: Option<Duration>,
    #[serde(rename = "queryFileLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_file_limit: Option<i32>,
    #[serde(rename = "queryLogSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_log_size: Option<i32>,
    #[serde(rename = "retentionCheckInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_check_interval: Option<Duration>,
    #[serde(rename = "snapshottedWalFilesToKeep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshotted_wal_files_to_keep: Option<i32>,
    #[serde(rename = "tableIndexCacheConcurrencyLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_index_cache_concurrency_limit: Option<i32>,
    #[serde(rename = "tableIndexCacheMaxEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_index_cache_max_entries: Option<i32>,
    #[serde(rename = "walMaxWriteBufferSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wal_max_write_buffer_size: Option<i32>,
    #[serde(rename = "walReplayConcurrencyLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wal_replay_concurrency_limit: Option<i32>,
    #[serde(rename = "walReplayFailOnError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wal_replay_fail_on_error: Option<bool>,
    #[serde(rename = "walSnapshotSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wal_snapshot_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PercentOrAbsoluteLong {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InfluxDBv3EnterpriseParameters {
    #[serde(rename = "catalogSyncInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_sync_interval: Option<Duration>,
    #[serde(rename = "compactionCheckInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction_check_interval: Option<Duration>,
    #[serde(rename = "compactionCleanupWait")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction_cleanup_wait: Option<Duration>,
    #[serde(rename = "compactionGen2Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction_gen2_duration: Option<Duration>,
    #[serde(rename = "compactionMaxNumFilesPerPlan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction_max_num_files_per_plan: Option<i32>,
    #[serde(rename = "compactionMultipliers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction_multipliers: Option<String>,
    #[serde(rename = "compactionRowLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction_row_limit: Option<i32>,
    #[serde(rename = "dataFusionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_config: Option<String>,
    #[serde(rename = "dataFusionMaxParquetFanout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_max_parquet_fanout: Option<i32>,
    #[serde(rename = "dataFusionNumThreads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_num_threads: Option<i32>,
    #[serde(rename = "dataFusionRuntimeDisableLifoSlot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_disable_lifo_slot: Option<bool>,
    #[serde(rename = "dataFusionRuntimeEventInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_event_interval: Option<i32>,
    #[serde(rename = "dataFusionRuntimeGlobalQueueInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_global_queue_interval: Option<i32>,
    #[serde(rename = "dataFusionRuntimeMaxBlockingThreads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_max_blocking_threads: Option<i32>,
    #[serde(rename = "dataFusionRuntimeMaxIoEventsPerTick")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_max_io_events_per_tick: Option<i32>,
    #[serde(rename = "dataFusionRuntimeThreadKeepAlive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_thread_keep_alive: Option<Duration>,
    #[serde(rename = "dataFusionRuntimeThreadPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_thread_priority: Option<i32>,
    #[serde(rename = "dataFusionRuntimeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_runtime_type: Option<String>,
    #[serde(rename = "dataFusionUseCachedParquetLoader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_fusion_use_cached_parquet_loader: Option<bool>,
    #[serde(rename = "dedicatedCompactor")]
    #[serde(default)]
    pub dedicated_compactor: bool,
    #[serde(rename = "deleteGracePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_grace_period: Option<Duration>,
    #[serde(rename = "disableParquetMemCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_parquet_mem_cache: Option<bool>,
    #[serde(rename = "distinctCacheEvictionInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_cache_eviction_interval: Option<Duration>,
    #[serde(rename = "distinctValueCacheDisableFromHistory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_value_cache_disable_from_history: Option<bool>,
    #[serde(rename = "execMemPoolBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_mem_pool_bytes: Option<PercentOrAbsoluteLong>,
    #[serde(rename = "forceSnapshotMemThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_snapshot_mem_threshold: Option<PercentOrAbsoluteLong>,
    #[serde(rename = "gen1Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gen1_duration: Option<Duration>,
    #[serde(rename = "gen1LookbackDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gen1_lookback_duration: Option<Duration>,
    #[serde(rename = "hardDeleteDefaultDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_delete_default_duration: Option<Duration>,
    #[serde(rename = "ingestQueryInstances")]
    #[serde(default)]
    pub ingest_query_instances: i32,
    #[serde(rename = "lastCacheEvictionInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_cache_eviction_interval: Option<Duration>,
    #[serde(rename = "lastValueCacheDisableFromHistory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_value_cache_disable_from_history: Option<bool>,
    #[serde(rename = "logFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_filter: Option<String>,
    #[serde(rename = "logFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_format: Option<String>,
    #[serde(rename = "maxHttpRequestSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_http_request_size: Option<i64>,
    #[serde(rename = "parquetMemCachePruneInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_mem_cache_prune_interval: Option<Duration>,
    #[serde(rename = "parquetMemCachePrunePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_mem_cache_prune_percentage: Option<f32>,
    #[serde(rename = "parquetMemCacheQueryPathDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_mem_cache_query_path_duration: Option<Duration>,
    #[serde(rename = "parquetMemCacheSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_mem_cache_size: Option<PercentOrAbsoluteLong>,
    #[serde(rename = "preemptiveCacheAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemptive_cache_age: Option<Duration>,
    #[serde(rename = "queryFileLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_file_limit: Option<i32>,
    #[serde(rename = "queryLogSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_log_size: Option<i32>,
    #[serde(rename = "queryOnlyInstances")]
    #[serde(default)]
    pub query_only_instances: i32,
    #[serde(rename = "replicationInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_interval: Option<Duration>,
    #[serde(rename = "retentionCheckInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_check_interval: Option<Duration>,
    #[serde(rename = "snapshottedWalFilesToKeep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshotted_wal_files_to_keep: Option<i32>,
    #[serde(rename = "tableIndexCacheConcurrencyLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_index_cache_concurrency_limit: Option<i32>,
    #[serde(rename = "tableIndexCacheMaxEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_index_cache_max_entries: Option<i32>,
    #[serde(rename = "walMaxWriteBufferSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wal_max_write_buffer_size: Option<i32>,
    #[serde(rename = "walReplayConcurrencyLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wal_replay_concurrency_limit: Option<i32>,
    #[serde(rename = "walReplayFailOnError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wal_replay_fail_on_error: Option<bool>,
    #[serde(rename = "walSnapshotSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wal_snapshot_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDbParameterGroupOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDbClusterInput {
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    pub db_cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDbClusterOutput {
    #[serde(rename = "dbClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDbInstanceInput {
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDbInstanceOutput {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_id: Option<String>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbParameterGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_identifier: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "influxAuthParametersSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influx_auth_parameters_secret_arn: Option<String>,
    #[serde(rename = "instanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_mode: Option<String>,
    #[serde(rename = "instanceModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_modes: Option<Vec<String>>,
    #[serde(rename = "lastMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_maintenance_time: Option<String>,
    #[serde(rename = "logDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "nextMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_maintenance_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "secondaryAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    #[serde(rename = "vpcSubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDbClusterInput {
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    pub db_cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDbClusterOutput {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "clusterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_configuration: Option<ClusterConfiguration>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbParameterGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_identifier: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "engineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    #[serde(rename = "failoverMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "influxAuthParametersSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influx_auth_parameters_secret_arn: Option<String>,
    #[serde(rename = "lastMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_maintenance_time: Option<String>,
    #[serde(rename = "logDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "nextMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_maintenance_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "readerEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    #[serde(rename = "vpcSubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterConfiguration {
    #[serde(rename = "dedicatedCompactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_compactor: Option<bool>,
    #[serde(rename = "ingestQueryInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_query_instances: Option<i32>,
    #[serde(rename = "queryOnlyInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_only_instances: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDbInstanceInput {
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDbInstanceOutput {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_id: Option<String>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbParameterGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_identifier: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "influxAuthParametersSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influx_auth_parameters_secret_arn: Option<String>,
    #[serde(rename = "instanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_mode: Option<String>,
    #[serde(rename = "instanceModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_modes: Option<Vec<String>>,
    #[serde(rename = "lastMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_maintenance_time: Option<String>,
    #[serde(rename = "logDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "nextMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_maintenance_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "secondaryAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    #[serde(rename = "vpcSubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDbParameterGroupInput {
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDbParameterGroupOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDbClustersInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDbClustersOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DbClusterSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DbClusterSummary {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "engineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "readerEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDbInstancesForClusterInput {
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    pub db_cluster_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDbInstancesForClusterOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DbInstanceForClusterSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DbInstanceForClusterSummary {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_mode: Option<String>,
    #[serde(rename = "instanceModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_modes: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDbInstancesInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDbInstancesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DbInstanceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DbInstanceSummary {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDbParameterGroupsInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDbParameterGroupsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DbParameterGroupSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DbParameterGroupSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootDbClusterInput {
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    pub db_cluster_id: String,
    #[serde(rename = "instanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootDbClusterOutput {
    #[serde(rename = "dbClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootDbInstanceInput {
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootDbInstanceOutput {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_id: Option<String>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbParameterGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_identifier: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "influxAuthParametersSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influx_auth_parameters_secret_arn: Option<String>,
    #[serde(rename = "instanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_mode: Option<String>,
    #[serde(rename = "instanceModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_modes: Option<Vec<String>>,
    #[serde(rename = "lastMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_maintenance_time: Option<String>,
    #[serde(rename = "logDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "nextMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_maintenance_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "secondaryAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    #[serde(rename = "vpcSubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDbClusterInput {
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    pub db_cluster_id: String,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbParameterGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_identifier: Option<String>,
    #[serde(rename = "failoverMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<String>,
    #[serde(rename = "logDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDbClusterOutput {
    #[serde(rename = "dbClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDbInstanceInput {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbParameterGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_identifier: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "logDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDbInstanceOutput {
    #[serde(rename = "allocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "dbClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_id: Option<String>,
    #[serde(rename = "dbInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    #[serde(rename = "dbParameterGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_identifier: Option<String>,
    #[serde(rename = "dbStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_storage_type: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "influxAuthParametersSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influx_auth_parameters_secret_arn: Option<String>,
    #[serde(rename = "instanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_mode: Option<String>,
    #[serde(rename = "instanceModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_modes: Option<Vec<String>>,
    #[serde(rename = "lastMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_maintenance_time: Option<String>,
    #[serde(rename = "logDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
    #[serde(rename = "maintenanceSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedule: Option<MaintenanceSchedule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "nextMaintenanceTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_maintenance_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "secondaryAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    #[serde(rename = "vpcSubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnet_ids: Option<Vec<String>>,
}
