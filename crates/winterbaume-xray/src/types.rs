#[derive(Debug, Clone)]
pub struct Group {
    pub group_name: String,
    pub group_arn: String,
    pub filter_expression: String,
    pub insights_configuration: Option<InsightsConfiguration>,
}

#[derive(Debug, Clone)]
pub struct InsightsConfiguration {
    pub insights_enabled: bool,
    pub notifications_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct TraceSummary {
    pub id: String,
    pub duration: f64,
    pub response_time: f64,
    pub has_fault: bool,
    pub has_error: bool,
    pub has_throttle: bool,
    pub http_method: String,
    pub http_url: String,
    pub http_status: i32,
}

/// A resource-based policy stored in X-Ray.
#[derive(Debug, Clone)]
pub struct ResourcePolicy {
    pub policy_name: String,
    pub policy_document: String,
    pub policy_revision_id: String,
    pub last_updated_time: f64,
}

/// Encryption configuration for X-Ray traces.
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    pub key_id: Option<String>,
    pub status: String,
    pub encryption_type: String,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        EncryptionConfig {
            key_id: None,
            status: "ACTIVE".to_string(),
            encryption_type: "NONE".to_string(),
        }
    }
}

/// A stored trace segment document alongside its parsed metadata.
#[derive(Debug, Clone)]
pub struct TraceSegmentRecord {
    /// The raw JSON document as submitted by the client.
    pub document: String,
    /// Segment ID extracted from the document.
    pub id: String,
    /// Trace ID extracted from the document.
    pub trace_id: String,
    /// Start time (epoch seconds) extracted from the document.
    pub start_time: f64,
    /// End time (epoch seconds), if present.
    pub end_time: Option<f64>,
    /// Whether this segment carried an `error` flag.
    pub has_error: bool,
    /// Whether this segment carried a `fault` flag.
    pub has_fault: bool,
    /// Whether this segment carried a `throttle` flag.
    pub has_throttle: bool,
}

/// A summary of a trace, returned by GetTraceSummaries.
#[derive(Debug, Clone)]
pub struct TraceSummaryData {
    pub id: String,
    pub duration: f64,
    pub has_error: bool,
    pub has_fault: bool,
    pub has_throttle: bool,
    pub start_time: f64,
}

/// A full trace with its segments, returned by BatchGetTraces.
#[derive(Debug, Clone)]
pub struct TraceData {
    pub id: String,
    pub duration: f64,
    pub segments: Vec<SegmentData>,
}

/// A single segment document within a trace.
#[derive(Debug, Clone)]
pub struct SegmentData {
    pub id: String,
    pub document: String,
}

/// Trace segment destination configuration.
#[derive(Debug, Clone, Default)]
pub struct TraceSegmentDestination {
    /// Destination service, e.g. `"XRay"` or `"CloudWatchLogs"`.
    pub destination: String,
}

/// An X-Ray indexing rule.
#[derive(Debug, Clone)]
pub struct IndexingRuleData {
    pub name: String,
    pub modified_at: f64,
    /// Desired sampling percentage for probabilistic rules.
    pub desired_sampling_percentage: Option<f64>,
}

/// A trace retrieval job.
#[derive(Debug, Clone)]
pub struct TraceRetrievalJob {
    pub retrieval_token: String,
    pub trace_ids: Vec<String>,
    pub status: String,
    pub created_at: f64,
}

/// A telemetry record as submitted by PutTelemetryRecords.
#[derive(Debug, Clone)]
pub struct TelemetryRecord {
    pub timestamp: f64,
    pub segments_received_count: Option<i32>,
    pub segments_sent_count: Option<i32>,
    pub segments_rejected_count: Option<i32>,
    pub segments_spillover_count: Option<i32>,
    pub backend_connection_errors: Option<i32>,
}

/// A sampling statistic summary.
#[derive(Debug, Clone)]
pub struct SamplingStatisticSummary {
    pub rule_name: String,
    pub timestamp: f64,
    pub request_count: i32,
    pub borrow_count: i32,
    pub sampled_count: i32,
}

/// A sampling rule.
#[derive(Debug, Clone)]
pub struct SamplingRuleData {
    pub rule_name: String,
    pub rule_arn: String,
    pub resource_arn: String,
    pub priority: i32,
    pub fixed_rate: f64,
    pub reservoir_size: i32,
    pub service_name: String,
    pub service_type: String,
    pub host: String,
    pub http_method: String,
    pub url_path: String,
    pub version: i32,
    pub created_at: f64,
    pub modified_at: f64,
}
