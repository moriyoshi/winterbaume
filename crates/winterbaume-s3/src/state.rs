use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use md5::{Digest, Md5};

use crate::types::{Bucket, CommonPrefix, DeleteMarker, Object};

/// In-memory state for the S3 service.
#[derive(Debug, Default)]
pub struct S3State {
    /// Buckets keyed by bucket name.
    pub buckets: HashMap<String, BucketState>,
}

/// State for a single bucket (metadata + objects).
#[derive(Debug, Clone)]
pub struct BucketState {
    pub bucket: Bucket,
    /// Current live objects keyed by object key (latest non-delete-marker version).
    pub objects: HashMap<String, Object>,
    /// Full version history per key, newest-first. Populated only when versioning
    /// has ever been enabled on the bucket.
    pub object_history: HashMap<String, Vec<Object>>,
    /// Delete markers per key, newest-first.
    pub delete_markers: HashMap<String, Vec<DeleteMarker>>,
    /// Raw XML lifecycle configuration (if set).
    pub lifecycle_configuration: Option<String>,
    /// Raw XML notification configuration (if set).
    pub notification_configuration: Option<String>,
    // FIX(terraform-e2e): All fields below were added for terraform compatibility.
    // Terraform's AWS provider queries ~15 sub-resource APIs (versioning, acl, policy,
    // tagging, encryption, cors, logging, etc.) when reading an S3 bucket. Without these
    // fields and their corresponding GET/PUT/DELETE handlers, terraform hangs or errors.
    /// Versioning status: None = never enabled, Some("Enabled") or Some("Suspended").
    pub versioning_status: Option<String>,
    /// Tags: key-value pairs.
    pub tags: HashMap<String, String>,
    /// Raw ACL XML (if set via PUT).
    pub acl: Option<String>,
    /// Bucket policy JSON (if set).
    pub policy: Option<String>,
    /// Accelerate configuration status.
    pub accelerate_status: Option<String>,
    /// Request payment configuration: "BucketOwner" or "Requester".
    pub request_payment_payer: String,
    /// CORS configuration XML (if set).
    pub cors_configuration: Option<String>,
    /// Server-side encryption configuration XML (if set).
    pub encryption_configuration: Option<String>,
    /// Logging configuration XML (if set).
    pub logging_configuration: Option<String>,
    /// Replication configuration XML (if set).
    pub replication_configuration: Option<String>,
    /// Inventory configurations XML keyed by inventory ID.
    pub inventory_configurations: HashMap<String, String>,
    /// Object lock configuration XML (if set).
    pub object_lock_configuration: Option<String>,
    /// Website configuration XML (if set).
    pub website_configuration: Option<String>,
    /// Public access block configuration.
    pub public_access_block: Option<PublicAccessBlockConfig>,
    /// Ownership controls rule.
    pub ownership_controls: Option<String>,
    /// In-flight multipart uploads keyed by upload ID.
    pub multipart_uploads: HashMap<String, MultipartUploadState>,
    /// Analytics configurations keyed by configuration ID.
    pub analytics_configurations: HashMap<String, String>,
    /// Intelligent tiering configurations keyed by configuration ID.
    pub intelligent_tiering_configurations: HashMap<String, String>,
    /// Metrics configurations keyed by configuration ID.
    pub metrics_configurations: HashMap<String, String>,
    /// ABAC status XML (if set).
    pub abac_status: Option<String>,
    /// Metadata table configuration XML (if set).
    pub metadata_table_configuration: Option<String>,
    /// Metadata configuration XML (if set).
    pub metadata_configuration: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MultipartUploadState {
    pub upload_id: String,
    pub key: String,
    pub initiated: DateTime<Utc>,
    pub content_type: String,
    pub storage_class: String,
    pub metadata: Vec<(String, String)>,
    pub parts: BTreeMap<i32, MultipartPartState>,
}

#[derive(Debug, Clone)]
pub struct MultipartPartState {
    pub part_number: i32,
    pub etag: String,
    /// Key used to retrieve the part body from the service's [`BlobStore`].
    pub blob_key: String,
    /// Size of the part body in bytes.
    pub content_length: u64,
    pub last_modified: DateTime<Utc>,
}

/// Public access block configuration for a bucket.
#[derive(Debug, Clone)]
pub struct PublicAccessBlockConfig {
    pub block_public_acls: bool,
    pub ignore_public_acls: bool,
    pub block_public_policy: bool,
    pub restrict_public_buckets: bool,
}

/// Error type for S3 operations.
#[derive(Debug, thiserror::Error)]
pub enum S3Error {
    #[error("The specified bucket is not valid. Bucket name must be between 3 and 63 characters.")]
    InvalidBucketName { resource: String },

    #[error("Your previous request to create the named bucket succeeded and you already own it.")]
    BucketAlreadyOwnedByYou { resource: String },

    #[error("The bucket you tried to delete is not empty.")]
    BucketNotEmpty { resource: String },

    #[error("The specified bucket does not exist.")]
    NoSuchBucket { resource: String },

    #[error("The specified key does not exist.")]
    NoSuchKey { resource: String },

    #[error("The specified multipart upload does not exist.")]
    NoSuchUpload { resource: String },

    #[error("Part number must be an integer between 1 and 10000.")]
    InvalidPartNumber { resource: String },

    #[error("Part {part_number} is missing or does not match the upload manifest.")]
    InvalidPart { part_number: i32, resource: String },

    #[error("Could not parse S3 request target.")]
    InvalidRequest { resource: String },

    #[error("Missing x-amz-copy-source header.")]
    MissingCopySourceHeader { resource: String },

    #[error("Invalid x-amz-copy-source header.")]
    InvalidCopySourceHeader { resource: String },

    #[error("The specified method is not allowed: {method}")]
    MethodNotAllowed { method: String, resource: String },

    #[error("{message}")]
    InternalError { message: String, resource: String },

    #[error("{message}")]
    MalformedXML { message: String, resource: String },

    #[error("Missing inventory configuration id.")]
    MissingInventoryConfigurationId { resource: String },

    #[error("The inventory configuration does not exist.")]
    NoSuchInventoryConfiguration { resource: String },

    #[error("The lifecycle configuration does not exist.")]
    NoSuchLifecycleConfiguration { resource: String },

    #[error("The bucket policy does not exist.")]
    NoSuchBucketPolicy { resource: String },

    #[error("The TagSet does not exist.")]
    NoSuchTagSet { resource: String },

    #[error("The CORS configuration does not exist.")]
    NoSuchCORSConfiguration { resource: String },

    #[error("Object Lock configuration does not exist for this bucket.")]
    ObjectLockConfigurationNotFoundError { resource: String },

    #[error("The replication configuration was not found.")]
    ReplicationConfigurationNotFoundError { resource: String },

    #[error("The specified bucket does not have a website configuration.")]
    NoSuchWebsiteConfiguration { resource: String },

    #[error("The public access block configuration was not found.")]
    NoSuchPublicAccessBlockConfiguration { resource: String },

    #[error("Missing analytics configuration id.")]
    MissingAnalyticsConfigurationId { resource: String },

    #[error("The analytics configuration does not exist.")]
    NoSuchAnalyticsConfiguration { resource: String },

    #[error("Missing intelligent tiering configuration id.")]
    MissingIntelligentTieringConfigurationId { resource: String },

    #[error("The intelligent tiering configuration does not exist.")]
    NoSuchIntelligentTieringConfiguration { resource: String },

    #[error("Missing metrics configuration id.")]
    MissingMetricsConfigurationId { resource: String },

    #[error("The metrics configuration does not exist.")]
    NoSuchMetricsConfiguration { resource: String },

    #[error("The metadata table configuration does not exist.")]
    NoSuchMetadataTableConfiguration { resource: String },

    #[error("The metadata configuration does not exist.")]
    NoSuchMetadataConfiguration { resource: String },

    #[error("Missing x-amz-rename-source header.")]
    MissingRenameSourceHeader { resource: String },

    #[error("At least one of the pre-conditions you specified did not hold.")]
    PreconditionFailed { resource: String },
}

impl S3Error {
    /// Return the resource path associated with this error.
    pub fn resource(&self) -> &str {
        match self {
            S3Error::InvalidBucketName { resource }
            | S3Error::BucketAlreadyOwnedByYou { resource }
            | S3Error::BucketNotEmpty { resource }
            | S3Error::NoSuchBucket { resource }
            | S3Error::NoSuchKey { resource }
            | S3Error::NoSuchUpload { resource }
            | S3Error::InvalidPartNumber { resource }
            | S3Error::InvalidRequest { resource }
            | S3Error::MissingCopySourceHeader { resource }
            | S3Error::InvalidCopySourceHeader { resource }
            | S3Error::MissingInventoryConfigurationId { resource }
            | S3Error::NoSuchInventoryConfiguration { resource }
            | S3Error::NoSuchLifecycleConfiguration { resource }
            | S3Error::NoSuchBucketPolicy { resource }
            | S3Error::NoSuchTagSet { resource }
            | S3Error::NoSuchCORSConfiguration { resource }
            | S3Error::ObjectLockConfigurationNotFoundError { resource }
            | S3Error::ReplicationConfigurationNotFoundError { resource }
            | S3Error::NoSuchWebsiteConfiguration { resource }
            | S3Error::NoSuchPublicAccessBlockConfiguration { resource }
            | S3Error::MissingAnalyticsConfigurationId { resource }
            | S3Error::NoSuchAnalyticsConfiguration { resource }
            | S3Error::MissingIntelligentTieringConfigurationId { resource }
            | S3Error::NoSuchIntelligentTieringConfiguration { resource }
            | S3Error::MissingMetricsConfigurationId { resource }
            | S3Error::NoSuchMetricsConfiguration { resource }
            | S3Error::NoSuchMetadataTableConfiguration { resource }
            | S3Error::NoSuchMetadataConfiguration { resource }
            | S3Error::MissingRenameSourceHeader { resource }
            | S3Error::PreconditionFailed { resource } => resource.as_str(),
            S3Error::InvalidPart { resource, .. }
            | S3Error::MethodNotAllowed { resource, .. }
            | S3Error::InternalError { resource, .. }
            | S3Error::MalformedXML { resource, .. } => resource.as_str(),
        }
    }
}

/// Outcome of a delete-object operation.
pub enum DeleteObjectOutcome {
    /// Object body was permanently deleted; caller must remove this specific version from
    /// the BlobStore using `delete_version(blob_key, blob_version_id)`.
    Deleted {
        blob_key: String,
        blob_version_id: String,
    },
    /// Versioning is active and no versionId was specified; a delete marker was inserted.
    DeleteMarkerCreated { version_id: String },
    /// Key did not exist (S3 delete is idempotent).
    NotFound,
}

/// Result of ListObjectsV2.
pub struct ListObjectsV2Result {
    pub objects: Vec<Object>,
    pub common_prefixes: Vec<CommonPrefix>,
    pub is_truncated: bool,
    pub next_continuation_token: Option<String>,
    pub key_count: usize,
}

impl S3State {
    /// List all buckets.
    pub fn list_buckets(&self) -> Vec<&Bucket> {
        let mut buckets: Vec<&Bucket> = self.buckets.values().map(|bs| &bs.bucket).collect();
        buckets.sort_by(|a, b| a.name.cmp(&b.name));
        buckets
    }

    pub fn create_bucket(&mut self, bucket_name: &str, region: &str) -> Result<&Bucket, S3Error> {
        if bucket_name.len() < 3 || bucket_name.len() > 63 {
            return Err(S3Error::InvalidBucketName {
                resource: format!("/{bucket_name}"),
            });
        }

        if self.buckets.contains_key(bucket_name) {
            return Err(S3Error::BucketAlreadyOwnedByYou {
                resource: format!("/{bucket_name}"),
            });
        }

        let bucket = Bucket {
            name: bucket_name.to_string(),
            region: region.to_string(),
            creation_date: Utc::now(),
        };

        self.buckets.insert(
            bucket_name.to_string(),
            BucketState {
                bucket,
                objects: HashMap::new(),
                object_history: HashMap::new(),
                delete_markers: HashMap::new(),
                lifecycle_configuration: None,
                notification_configuration: None,
                versioning_status: None,
                tags: HashMap::new(),
                acl: None,
                policy: None,
                accelerate_status: None,
                request_payment_payer: "BucketOwner".to_string(),
                cors_configuration: None,
                encryption_configuration: None,
                logging_configuration: None,
                replication_configuration: None,
                inventory_configurations: HashMap::new(),
                object_lock_configuration: None,
                website_configuration: None,
                public_access_block: None,
                ownership_controls: None,
                multipart_uploads: HashMap::new(),
                analytics_configurations: HashMap::new(),
                intelligent_tiering_configurations: HashMap::new(),
                metrics_configurations: HashMap::new(),
                abac_status: None,
                metadata_table_configuration: None,
                metadata_configuration: None,
            },
        );

        Ok(&self.buckets.get(bucket_name).unwrap().bucket)
    }

    pub fn head_bucket(&self, bucket_name: &str) -> Result<&Bucket, S3Error> {
        self.buckets
            .get(bucket_name)
            .map(|bs| &bs.bucket)
            .ok_or_else(|| no_such_bucket(bucket_name))
    }

    pub fn delete_bucket(&mut self, bucket_name: &str) -> Result<(), S3Error> {
        let bucket_state = self
            .buckets
            .get(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;

        if !bucket_state.objects.is_empty()
            || !bucket_state.object_history.is_empty()
            || !bucket_state.delete_markers.is_empty()
            || !bucket_state.multipart_uploads.is_empty()
        {
            return Err(S3Error::BucketNotEmpty {
                resource: format!("/{bucket_name}"),
            });
        }

        self.buckets.remove(bucket_name);
        Ok(())
    }

    /// Store an object with a pre-computed blob key, blob version ID, etag, and content length.
    ///
    /// The caller is responsible for writing the body to the BlobStore via
    /// `put_versioned_from(blob_key, reader)` (which returns `blob_version_id`) before
    /// calling this method.
    ///
    /// When versioning is `Enabled` a new UUID version ID is generated and the
    /// previous versions are preserved in `object_history`.  Otherwise the
    /// object is stored with `version_id = "null"` and any previous version is
    /// replaced.
    pub fn put_object(
        &mut self,
        bucket_name: &str,
        key: &str,
        blob_key: String,
        blob_version_id: String,
        etag: String,
        content_length: u64,
        content_type: Option<&str>,
        metadata: Vec<(String, String)>,
    ) -> Result<&Object, S3Error> {
        let bucket_state = self
            .buckets
            .get_mut(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;

        let ct = content_type.unwrap_or("binary/octet-stream");

        let versioning_enabled = bucket_state.versioning_status.as_deref() == Some("Enabled");
        let version_id = if versioning_enabled {
            uuid::Uuid::new_v4().to_string()
        } else {
            "null".to_string()
        };

        let object = Object {
            key: key.to_string(),
            blob_key,
            blob_version_id,
            content_length,
            content_type: ct.to_string(),
            etag,
            last_modified: Utc::now(),
            storage_class: "STANDARD".to_string(),
            metadata,
            tags: HashMap::new(),
            acl: None,
            legal_hold_status: None,
            retention_mode: None,
            retain_until_date: None,
            version_id,
        };

        if versioning_enabled {
            // Prepend new version to history.
            bucket_state
                .object_history
                .entry(key.to_string())
                .or_default()
                .insert(0, object.clone());
            // A new version clears any delete markers on this key.
            bucket_state.delete_markers.remove(key);
        }

        bucket_state.objects.insert(key.to_string(), object);
        Ok(bucket_state.objects.get(key).unwrap())
    }

    pub fn get_object(
        &self,
        bucket_name: &str,
        key: &str,
        version_id: Option<&str>,
    ) -> Result<&Object, S3Error> {
        let bucket_state = self
            .buckets
            .get(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;

        let no_such_key = || S3Error::NoSuchKey {
            resource: format!("/{bucket_name}/{key}"),
        };

        if let Some(vid) = version_id {
            // Look up a specific version in history.
            bucket_state
                .object_history
                .get(key)
                .and_then(|versions| versions.iter().find(|o| o.version_id == vid))
                .ok_or_else(no_such_key)
        } else {
            // Return the current live version.
            bucket_state.objects.get(key).ok_or_else(no_such_key)
        }
    }

    pub fn head_object(
        &self,
        bucket_name: &str,
        key: &str,
        version_id: Option<&str>,
    ) -> Result<&Object, S3Error> {
        self.get_object(bucket_name, key, version_id)
    }

    /// Delete an object or create a delete marker, depending on versioning state.
    ///
    /// - `version_id = None` with versioning active → insert a delete marker,
    ///   remove from the live `objects` map, return `DeleteMarkerCreated`.
    /// - `version_id = Some(id)` → permanently delete that specific version
    ///   (or delete marker); return `Deleted` or `NotFound`.
    /// - Versioning off / `version_id = None` → remove the live object and
    ///   return `Deleted` (S3 delete is idempotent; `NotFound` when absent).
    pub fn delete_object(
        &mut self,
        bucket_name: &str,
        key: &str,
        version_id: Option<&str>,
    ) -> Result<DeleteObjectOutcome, S3Error> {
        let bucket_state = self
            .buckets
            .get_mut(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;

        let versioning_active = matches!(
            bucket_state.versioning_status.as_deref(),
            Some("Enabled") | Some("Suspended")
        );

        if let Some(vid) = version_id {
            // Permanent deletion of a specific version or delete marker.

            // Try to remove from object_history first.
            if let Some(versions) = bucket_state.object_history.get_mut(key) {
                if let Some(pos) = versions.iter().position(|o| o.version_id == vid) {
                    let removed = versions.remove(pos);
                    if versions.is_empty() {
                        bucket_state.object_history.remove(key);
                    }
                    // If this was the current live version, promote the next one.
                    if bucket_state
                        .objects
                        .get(key)
                        .map(|o| o.version_id == vid)
                        .unwrap_or(false)
                    {
                        // Promote next version from history (if any).
                        if let Some(next) = bucket_state
                            .object_history
                            .get(key)
                            .and_then(|v| v.first())
                            .cloned()
                        {
                            bucket_state.objects.insert(key.to_string(), next);
                        } else {
                            bucket_state.objects.remove(key);
                        }
                    }
                    return Ok(DeleteObjectOutcome::Deleted {
                        blob_key: removed.blob_key.clone(),
                        blob_version_id: removed.blob_version_id.clone(),
                    });
                }
            }

            // Try to remove from delete_markers.
            if let Some(markers) = bucket_state.delete_markers.get_mut(key) {
                if let Some(pos) = markers.iter().position(|m| m.version_id == vid) {
                    markers.remove(pos);
                    if markers.is_empty() {
                        bucket_state.delete_markers.remove(key);
                    }
                    // Removing a delete marker resurfaces the previous live version if any.
                    if !bucket_state.objects.contains_key(key) {
                        if let Some(prev) = bucket_state
                            .object_history
                            .get(key)
                            .and_then(|v| v.first())
                            .cloned()
                        {
                            bucket_state.objects.insert(key.to_string(), prev);
                        }
                    }
                    // Deleting a delete marker is reported as a deletion with no blob.
                    // We signal NotFound here to avoid the caller trying to delete a blob.
                    return Ok(DeleteObjectOutcome::NotFound);
                }
            }

            // Neither a version nor a delete marker matched — also check non-versioned live object.
            if let Some(obj) = bucket_state.objects.get(key) {
                if obj.version_id == vid {
                    let removed = bucket_state.objects.remove(key).unwrap();
                    return Ok(DeleteObjectOutcome::Deleted {
                        blob_key: removed.blob_key,
                        blob_version_id: removed.blob_version_id,
                    });
                }
            }

            return Ok(DeleteObjectOutcome::NotFound);
        }

        // No version_id specified.
        if versioning_active {
            let new_version_id = if bucket_state.versioning_status.as_deref() == Some("Enabled") {
                uuid::Uuid::new_v4().to_string()
            } else {
                // Suspended: use "null" version_id for delete markers too.
                "null".to_string()
            };
            let marker = DeleteMarker {
                key: key.to_string(),
                version_id: new_version_id.clone(),
                last_modified: Utc::now(),
            };
            bucket_state
                .delete_markers
                .entry(key.to_string())
                .or_default()
                .insert(0, marker);
            bucket_state.objects.remove(key);
            return Ok(DeleteObjectOutcome::DeleteMarkerCreated {
                version_id: new_version_id,
            });
        }

        // Versioning off — simple delete.
        match bucket_state.objects.remove(key) {
            Some(obj) => Ok(DeleteObjectOutcome::Deleted {
                blob_key: obj.blob_key,
                blob_version_id: obj.blob_version_id,
            }),
            None => Ok(DeleteObjectOutcome::NotFound),
        }
    }

    pub fn list_objects_v2(
        &self,
        bucket_name: &str,
        prefix: Option<&str>,
        delimiter: Option<&str>,
        max_keys: usize,
        continuation_token: Option<&str>,
        start_after: Option<&str>,
    ) -> Result<ListObjectsV2Result, S3Error> {
        let bucket_state = self
            .buckets
            .get(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;

        let prefix = prefix.unwrap_or("");

        // Determine the starting point for pagination
        let start_after_key = continuation_token.or(start_after).unwrap_or("");

        // Collect and filter objects
        let mut keys: Vec<&Object> = Vec::new();
        let mut common_prefixes: Vec<String> = Vec::new();

        let mut sorted_keys: Vec<&String> = bucket_state.objects.keys().collect();
        sorted_keys.sort();

        for obj_key in sorted_keys {
            if !obj_key.starts_with(prefix) {
                continue;
            }
            if !start_after_key.is_empty() && obj_key.as_str() <= start_after_key {
                continue;
            }

            if let Some(delim) = delimiter {
                let after_prefix = &obj_key[prefix.len()..];
                if let Some(pos) = after_prefix.find(delim) {
                    let cp = format!("{}{}{}", prefix, &after_prefix[..pos], delim);
                    if !common_prefixes.contains(&cp) {
                        common_prefixes.push(cp);
                    }
                    continue;
                }
            }

            keys.push(bucket_state.objects.get(obj_key).unwrap());
        }

        // Combine for truncation counting
        let total_count = keys.len() + common_prefixes.len();
        let is_truncated = total_count > max_keys;

        // Truncate
        let key_count;
        let next_continuation_token;

        if is_truncated {
            // We need to truncate the combined results to max_keys
            // For simplicity, truncate keys first, then common_prefixes
            let mut remaining = max_keys;
            if keys.len() > remaining {
                keys.truncate(remaining);
                remaining = 0;
            } else {
                remaining -= keys.len();
            }
            if common_prefixes.len() > remaining {
                common_prefixes.truncate(remaining);
            }
            key_count = keys.len() + common_prefixes.len();

            // The continuation token is the last key
            let last_key = keys
                .last()
                .map(|o| o.key.clone())
                .or_else(|| common_prefixes.last().cloned());
            next_continuation_token = last_key;
        } else {
            key_count = total_count;
            next_continuation_token = None;
        }

        Ok(ListObjectsV2Result {
            objects: keys.into_iter().cloned().collect(),
            common_prefixes: common_prefixes
                .into_iter()
                .map(|p| CommonPrefix { prefix: p })
                .collect(),
            is_truncated,
            next_continuation_token,
            key_count,
        })
    }

    pub fn put_bucket_lifecycle_configuration(
        &mut self,
        bucket_name: &str,
        xml: String,
    ) -> Result<(), S3Error> {
        let bs = self
            .buckets
            .get_mut(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        bs.lifecycle_configuration = Some(xml);
        Ok(())
    }

    pub fn get_bucket_lifecycle_configuration(
        &self,
        bucket_name: &str,
    ) -> Result<Option<&str>, S3Error> {
        let bs = self
            .buckets
            .get(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        Ok(bs.lifecycle_configuration.as_deref())
    }

    pub fn put_bucket_notification_configuration(
        &mut self,
        bucket_name: &str,
        xml: String,
    ) -> Result<(), S3Error> {
        let bs = self
            .buckets
            .get_mut(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        bs.notification_configuration = Some(xml);
        Ok(())
    }

    pub fn get_bucket_notification_configuration(
        &self,
        bucket_name: &str,
    ) -> Result<Option<&str>, S3Error> {
        let bs = self
            .buckets
            .get(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        Ok(bs.notification_configuration.as_deref())
    }

    pub fn create_multipart_upload(
        &mut self,
        bucket_name: &str,
        key: &str,
        content_type: Option<&str>,
        metadata: Vec<(String, String)>,
        storage_class: Option<&str>,
    ) -> Result<MultipartUploadState, S3Error> {
        let bucket_state = self
            .buckets
            .get_mut(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        let upload = MultipartUploadState {
            upload_id: uuid::Uuid::new_v4().to_string(),
            key: key.to_string(),
            initiated: Utc::now(),
            content_type: content_type.unwrap_or("binary/octet-stream").to_string(),
            storage_class: storage_class.unwrap_or("STANDARD").to_string(),
            metadata,
            parts: BTreeMap::new(),
        };
        bucket_state
            .multipart_uploads
            .insert(upload.upload_id.clone(), upload.clone());
        Ok(upload)
    }

    pub fn list_multipart_uploads(
        &self,
        bucket_name: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<MultipartUploadState>, S3Error> {
        let bucket_state = self
            .buckets
            .get(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        let prefix = prefix.unwrap_or("");
        let mut uploads = bucket_state
            .multipart_uploads
            .values()
            .filter(|upload| upload.key.starts_with(prefix))
            .cloned()
            .collect::<Vec<_>>();
        uploads.sort_by(|left, right| {
            left.key
                .cmp(&right.key)
                .then(left.upload_id.cmp(&right.upload_id))
        });
        Ok(uploads)
    }

    /// Store a multipart part with a pre-computed blob key, etag, and content length.
    ///
    /// The caller is responsible for writing the part body to the BlobStore
    /// under `blob_key` before calling this method.
    pub fn upload_part(
        &mut self,
        bucket_name: &str,
        key: &str,
        upload_id: &str,
        part_number: i32,
        blob_key: String,
        etag: String,
        content_length: u64,
    ) -> Result<MultipartPartState, S3Error> {
        if !(1..=10_000).contains(&part_number) {
            return Err(invalid_part_number(bucket_name, key));
        }
        let bucket_state = self
            .buckets
            .get_mut(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        let upload = bucket_state
            .multipart_uploads
            .get_mut(upload_id)
            .ok_or_else(|| no_such_upload(bucket_name, key, upload_id))?;
        if upload.key != key {
            return Err(no_such_upload(bucket_name, key, upload_id));
        }
        let part = MultipartPartState {
            part_number,
            etag,
            blob_key,
            content_length,
            last_modified: Utc::now(),
        };
        upload.parts.insert(part_number, part.clone());
        Ok(part)
    }

    pub fn list_parts(
        &self,
        bucket_name: &str,
        key: &str,
        upload_id: &str,
    ) -> Result<Vec<MultipartPartState>, S3Error> {
        let bucket_state = self
            .buckets
            .get(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        let upload = bucket_state
            .multipart_uploads
            .get(upload_id)
            .ok_or_else(|| no_such_upload(bucket_name, key, upload_id))?;
        if upload.key != key {
            return Err(no_such_upload(bucket_name, key, upload_id));
        }
        Ok(upload.parts.values().cloned().collect())
    }

    /// Abort a multipart upload and return the blob_keys of all its parts
    /// so the caller can clean up the blobs from the BlobStore.
    pub fn abort_multipart_upload(
        &mut self,
        bucket_name: &str,
        key: &str,
        upload_id: &str,
    ) -> Result<Vec<String>, S3Error> {
        let bucket_state = self
            .buckets
            .get_mut(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        let upload = bucket_state
            .multipart_uploads
            .get(upload_id)
            .ok_or_else(|| no_such_upload(bucket_name, key, upload_id))?;
        if upload.key != key {
            return Err(no_such_upload(bucket_name, key, upload_id));
        }
        let part_blob_keys: Vec<String> =
            upload.parts.values().map(|p| p.blob_key.clone()).collect();
        bucket_state.multipart_uploads.remove(upload_id);
        Ok(part_blob_keys)
    }

    /// Validate the completed parts list and return the ordered parts for
    /// the caller to read blobs from and concatenate.
    ///
    /// This does NOT modify state; call [`finalize_multipart_upload`] after
    /// the caller has stored the combined blob in the BlobStore.
    pub fn select_multipart_parts(
        &self,
        bucket_name: &str,
        key: &str,
        upload_id: &str,
        completed_parts: Option<&Vec<(i32, Option<String>)>>,
    ) -> Result<Vec<MultipartPartState>, S3Error> {
        let bucket_state = self
            .buckets
            .get(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        let upload = bucket_state
            .multipart_uploads
            .get(upload_id)
            .ok_or_else(|| no_such_upload(bucket_name, key, upload_id))?;
        if upload.key != key {
            return Err(no_such_upload(bucket_name, key, upload_id));
        }

        let selected = if let Some(parts) = completed_parts {
            let mut selected = Vec::new();
            for (part_number, expected_etag) in parts {
                let part = upload
                    .parts
                    .get(part_number)
                    .cloned()
                    .ok_or_else(|| invalid_part(bucket_name, key, upload_id, *part_number))?;
                if let Some(expected_etag) = expected_etag {
                    if &part.etag != expected_etag {
                        return Err(invalid_part(bucket_name, key, upload_id, *part_number));
                    }
                }
                selected.push(part);
            }
            selected
        } else {
            upload.parts.values().cloned().collect::<Vec<_>>()
        };

        Ok(selected)
    }

    /// Create the final object from a completed multipart upload.
    ///
    /// The caller must call [`select_multipart_parts`] first to validate parts
    /// and then write the combined blob to the BlobStore before calling this.
    ///
    /// Returns the new object. The old multipart upload is removed from state.
    pub fn finalize_multipart_upload(
        &mut self,
        bucket_name: &str,
        key: &str,
        upload_id: &str,
        final_blob_key: String,
        final_blob_version_id: String,
        final_etag: String,
        final_content_length: u64,
    ) -> Result<Object, S3Error> {
        let bucket_state = self
            .buckets
            .get_mut(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;
        let upload = bucket_state
            .multipart_uploads
            .remove(upload_id)
            .ok_or_else(|| no_such_upload(bucket_name, key, upload_id))?;
        if upload.key != key {
            // Put the upload back since we removed it prematurely.
            bucket_state
                .multipart_uploads
                .insert(upload_id.to_string(), upload);
            return Err(no_such_upload(bucket_name, key, upload_id));
        }

        let versioning_enabled = bucket_state.versioning_status.as_deref() == Some("Enabled");
        let version_id = if versioning_enabled {
            uuid::Uuid::new_v4().to_string()
        } else {
            "null".to_string()
        };

        let object = Object {
            key: key.to_string(),
            blob_key: final_blob_key,
            blob_version_id: final_blob_version_id,
            content_length: final_content_length,
            content_type: upload.content_type,
            etag: final_etag,
            last_modified: Utc::now(),
            storage_class: upload.storage_class,
            metadata: upload.metadata,
            tags: HashMap::new(),
            acl: None,
            legal_hold_status: None,
            retention_mode: None,
            retain_until_date: None,
            version_id,
        };

        if versioning_enabled {
            bucket_state
                .object_history
                .entry(key.to_string())
                .or_default()
                .insert(0, object.clone());
            bucket_state.delete_markers.remove(key);
        }

        bucket_state.objects.insert(key.to_string(), object.clone());
        Ok(object)
    }

    pub fn list_object_versions(
        &self,
        bucket_name: &str,
        prefix: Option<&str>,
        delimiter: Option<&str>,
        max_keys: usize,
    ) -> Result<
        (
            Vec<(Object, bool)>,
            Vec<DeleteMarker>,
            Vec<CommonPrefix>,
            bool,
        ),
        S3Error,
    > {
        let bucket_state = self
            .buckets
            .get(bucket_name)
            .ok_or_else(|| no_such_bucket(bucket_name))?;

        let prefix = prefix.unwrap_or("");
        let versioning_ever_enabled = !bucket_state.object_history.is_empty()
            || !bucket_state.delete_markers.is_empty()
            || bucket_state.versioning_status.is_some();

        let mut versions: Vec<(Object, bool)> = Vec::new();
        let mut delete_markers_out: Vec<DeleteMarker> = Vec::new();
        let mut common_prefixes: Vec<CommonPrefix> = Vec::new();

        // Collect all keys involved (from objects, history, and delete_markers).
        let mut all_keys: Vec<&String> = bucket_state
            .objects
            .keys()
            .chain(bucket_state.object_history.keys())
            .chain(bucket_state.delete_markers.keys())
            .collect();
        all_keys.sort();
        all_keys.dedup();

        for object_key in all_keys {
            if !object_key.starts_with(prefix) {
                continue;
            }
            if let Some(delimiter) = delimiter {
                let after_prefix = &object_key[prefix.len()..];
                if let Some(pos) = after_prefix.find(delimiter) {
                    let common_prefix = format!("{}{}{}", prefix, &after_prefix[..pos], delimiter);
                    if !common_prefixes
                        .iter()
                        .any(|entry| entry.prefix == common_prefix)
                    {
                        common_prefixes.push(CommonPrefix {
                            prefix: common_prefix,
                        });
                    }
                    continue;
                }
            }

            if versioning_ever_enabled {
                // Determine the current live version_id (if any).
                let live_version_id = bucket_state
                    .objects
                    .get(object_key.as_str())
                    .map(|o| o.version_id.clone());

                // Emit all historical versions for this key.
                if let Some(history) = bucket_state.object_history.get(object_key.as_str()) {
                    for obj in history {
                        let is_latest = live_version_id.as_deref() == Some(&obj.version_id);
                        versions.push((obj.clone(), is_latest));
                    }
                } else if let Some(obj) = bucket_state.objects.get(object_key.as_str()) {
                    // Versioning enabled but no history entry yet (edge case).
                    versions.push((obj.clone(), true));
                }

                // Emit delete markers for this key.
                if let Some(markers) = bucket_state.delete_markers.get(object_key.as_str()) {
                    for marker in markers {
                        delete_markers_out.push(marker.clone());
                    }
                }
            } else {
                // Versioning never active: return current objects with version_id = "null".
                if let Some(obj) = bucket_state.objects.get(object_key.as_str()) {
                    versions.push((obj.clone(), true));
                }
            }
        }

        let total = versions.len() + delete_markers_out.len() + common_prefixes.len();
        let is_truncated = total > max_keys;
        if is_truncated {
            let keep = max_keys.min(versions.len());
            versions.truncate(keep);
            if keep == max_keys {
                delete_markers_out.clear();
            } else {
                let remaining = max_keys - keep;
                delete_markers_out.truncate(remaining);
            }
        }

        Ok((versions, delete_markers_out, common_prefixes, is_truncated))
    }
}

pub fn no_such_bucket(bucket_name: &str) -> S3Error {
    S3Error::NoSuchBucket {
        resource: format!("/{bucket_name}"),
    }
}

pub fn no_such_upload(bucket_name: &str, key: &str, upload_id: &str) -> S3Error {
    S3Error::NoSuchUpload {
        resource: format!("/{bucket_name}/{key}?uploadId={upload_id}"),
    }
}

fn invalid_part_number(bucket_name: &str, key: &str) -> S3Error {
    S3Error::InvalidPartNumber {
        resource: format!("/{bucket_name}/{key}"),
    }
}

fn invalid_part(bucket_name: &str, key: &str, upload_id: &str, part_number: i32) -> S3Error {
    S3Error::InvalidPart {
        part_number,
        resource: format!("/{bucket_name}/{key}?uploadId={upload_id}"),
    }
}

/// Compute an S3-style ETag (MD5 hex, double-quoted).
pub fn compute_etag(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("\"{:x}\"", result)
}

/// Compute the MD5 hex string of `data` without surrounding quotes.
///
/// Used to build multipart ETags of the form `"{md5(partEtags)}-N"`.
pub fn compute_etag_raw(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_etag() {
        let etag = compute_etag(b"hello");
        assert!(etag.starts_with('"'));
        assert!(etag.ends_with('"'));
        assert_eq!(etag, "\"5d41402abc4b2a76b9719d911017c592\"");
    }
}
