use std::collections::HashMap;

/// An IVS channel.
#[derive(Debug, Clone)]
pub struct Channel {
    pub arn: String,
    pub name: String,
    pub latency_mode: String,
    pub channel_type: String,
    pub authorized: bool,
    pub tags: HashMap<String, String>,
}

/// An IVS stream key.
#[derive(Debug, Clone)]
pub struct StreamKey {
    pub arn: String,
    pub channel_arn: String,
    pub value: String,
    pub tags: HashMap<String, String>,
}

/// An IVS recording configuration.
#[derive(Debug, Clone)]
pub struct RecordingConfiguration {
    pub arn: String,
    pub name: String,
    pub destination_configuration: RecordingDestination,
    pub state: String,
    pub tags: HashMap<String, String>,
}

/// Recording destination.
#[derive(Debug, Clone)]
pub struct RecordingDestination {
    pub s3_bucket_name: Option<String>,
}

/// An IVS playback key pair.
#[derive(Debug, Clone)]
pub struct PlaybackKeyPair {
    pub arn: String,
    pub name: String,
    pub fingerprint: String,
    pub tags: HashMap<String, String>,
}

/// An IVS playback restriction policy.
#[derive(Debug, Clone)]
pub struct PlaybackRestrictionPolicy {
    pub arn: String,
    pub name: String,
    pub allowed_countries: Vec<String>,
    pub allowed_origins: Vec<String>,
    pub enable_strict_origin_enforcement: bool,
    pub tags: HashMap<String, String>,
}

/// Resource type for tagging lookups.
#[derive(Debug, Clone)]
pub enum IvsResource {
    Channel,
    StreamKey,
    RecordingConfiguration,
    PlaybackKeyPair,
    PlaybackRestrictionPolicy,
}
