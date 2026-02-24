use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// An S3 bucket.
#[derive(Debug, Clone)]
pub struct Bucket {
    pub name: String,
    pub region: String,
    pub creation_date: DateTime<Utc>,
}

/// An S3 object.
#[derive(Debug, Clone)]
pub struct Object {
    pub key: String,
    /// Key used to retrieve the body from the service's [`BlobStore`].
    pub blob_key: String,
    /// Size of the object body in bytes.
    pub content_length: u64,
    pub content_type: String,
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub storage_class: String,
    pub metadata: Vec<(String, String)>,
    pub tags: HashMap<String, String>,
    pub acl: Option<String>,
    pub legal_hold_status: Option<String>,
    pub retention_mode: Option<String>,
    pub retain_until_date: Option<String>,
    /// Version ID: `"null"` when versioning is disabled/suspended, otherwise a UUID.
    pub version_id: String,
    /// BlobStore internal version ID for this specific version's blob data.
    ///
    /// Used with [`BlobStore::get_version_reader`] and [`BlobStore::delete_version`]
    /// to access or remove the exact blob without touching other versions stored
    /// under the same stable [`blob_key`].
    pub blob_version_id: String,
}

/// A delete marker in a versioning-enabled bucket.
#[derive(Debug, Clone)]
pub struct DeleteMarker {
    pub key: String,
    pub version_id: String,
    pub last_modified: DateTime<Utc>,
}

impl Object {
    pub fn size(&self) -> usize {
        self.content_length as usize
    }
}

/// A "common prefix" entry from ListObjectsV2 (folder-like grouping).
#[derive(Debug, Clone)]
pub struct CommonPrefix {
    pub prefix: String,
}
