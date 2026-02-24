use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Fragment {
    pub fragment_number: String,
    pub fragment_size_in_bytes: i64,
    pub producer_timestamp: DateTime<Utc>,
    pub server_timestamp: DateTime<Utc>,
    pub fragment_length_in_milliseconds: i64,
}

#[derive(Debug, Clone)]
pub struct StreamData {
    pub stream_name: String,
    pub stream_arn: String,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Clone)]
pub struct StreamingSession {
    pub session_url: String,
    pub session_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ClipResult {
    pub content_type: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ImageResult {
    pub image_content: String,
    pub timestamp: DateTime<Utc>,
    pub error: Option<ImageError>,
}

#[derive(Debug, Clone)]
pub struct ImageError {
    pub error_code: String,
    pub error_message: String,
}
