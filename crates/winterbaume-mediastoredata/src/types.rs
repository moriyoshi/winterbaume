use bytes::Bytes;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct MediaStoreObject {
    /// The path (key) of the object, e.g. "folder/file.mp4"
    pub path: String,
    /// The content type (MIME type)
    pub content_type: String,
    /// The raw body bytes
    pub body: Bytes,
    /// Content length in bytes
    pub content_length: u64,
    /// ETag for the object
    pub etag: String,
    /// Last modified timestamp
    pub last_modified: DateTime<Utc>,
    /// Cache control header value
    pub cache_control: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub name: String,
    pub item_type: ItemType,
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
    pub etag: Option<String>,
    pub last_modified: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Object,
    Folder,
}
