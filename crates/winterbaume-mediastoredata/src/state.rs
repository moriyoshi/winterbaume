use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use bytes::Bytes;
use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct MediaStoreDataState {
    /// Objects stored by path (without leading slash)
    pub objects: HashMap<String, MediaStoreObject>,
}

#[derive(Debug, Error)]
pub enum MediaStoreDataError {
    #[error("The specified object was not found: {path}")]
    ObjectNotFound { path: String },
}

impl MediaStoreDataState {
    pub fn put_object(
        &mut self,
        path: &str,
        body: Bytes,
        content_type: Option<&str>,
        cache_control: Option<&str>,
    ) -> Result<&MediaStoreObject, MediaStoreDataError> {
        let content_length = body.len() as u64;
        let etag = compute_etag(&body);
        let ct = content_type
            .unwrap_or("application/octet-stream")
            .to_string();

        let obj = MediaStoreObject {
            path: path.to_string(),
            content_type: ct,
            body,
            content_length,
            etag,
            last_modified: Utc::now(),
            cache_control: cache_control.map(|s| s.to_string()),
        };

        self.objects.insert(path.to_string(), obj);
        Ok(self.objects.get(path).unwrap())
    }

    pub fn get_object(&self, path: &str) -> Result<&MediaStoreObject, MediaStoreDataError> {
        self.objects
            .get(path)
            .ok_or_else(|| MediaStoreDataError::ObjectNotFound {
                path: path.to_string(),
            })
    }

    pub fn delete_object(&mut self, path: &str) -> Result<(), MediaStoreDataError> {
        if self.objects.remove(path).is_none() {
            return Err(MediaStoreDataError::ObjectNotFound {
                path: path.to_string(),
            });
        }
        Ok(())
    }

    pub fn describe_object(&self, path: &str) -> Result<&MediaStoreObject, MediaStoreDataError> {
        self.objects
            .get(path)
            .ok_or_else(|| MediaStoreDataError::ObjectNotFound {
                path: path.to_string(),
            })
    }

    pub fn list_items(&self, path: Option<&str>) -> Vec<ItemInfo> {
        let prefix = match path {
            Some(p) => {
                let p = p.trim_matches('/');
                if p.is_empty() {
                    String::new()
                } else {
                    format!("{p}/")
                }
            }
            None => String::new(),
        };

        let mut seen_folders: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut items: Vec<ItemInfo> = Vec::new();

        for (key, obj) in &self.objects {
            if !key.starts_with(&prefix) {
                continue;
            }
            let remainder = &key[prefix.len()..];
            if remainder.is_empty() {
                continue;
            }
            if let Some(slash_pos) = remainder.find('/') {
                // This is a "subfolder"
                let folder_name = &remainder[..slash_pos];
                if seen_folders.insert(folder_name.to_string()) {
                    items.push(ItemInfo {
                        name: folder_name.to_string(),
                        item_type: ItemType::Folder,
                        content_type: None,
                        content_length: None,
                        etag: None,
                        last_modified: None,
                    });
                }
            } else {
                // Direct child object
                items.push(ItemInfo {
                    name: remainder.to_string(),
                    item_type: ItemType::Object,
                    content_type: Some(obj.content_type.clone()),
                    content_length: Some(obj.content_length),
                    etag: Some(obj.etag.clone()),
                    last_modified: Some(obj.last_modified),
                });
            }
        }

        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }
}

fn compute_etag(data: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    let h = hasher.finish();
    format!("{h:016x}")
}
