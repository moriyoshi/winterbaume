use std::collections::HashMap;

use crate::types::{Index, TagsMap, Vector, VectorBucket, VectorBucketPolicy};

#[derive(Debug, thiserror::Error)]
pub enum S3VectorsError {
    #[error("vectorBucketName or vectorBucketArn is required")]
    BucketIdentifierRequired,
    #[error("bucket and index identifiers are required")]
    IndexIdentifierRequired,
    #[error("Vector bucket '{0}' already exists")]
    BucketAlreadyExists(String),
    #[error("Vector bucket '{0}' contains one or more indexes; delete them first")]
    BucketNotEmpty(String),
    #[error("Vector bucket '{0}' not found")]
    BucketNotFound(String),
    #[error("Index '{0}' already exists in bucket '{1}'")]
    IndexAlreadyExists(String, String),
    #[error("Index '{0}' not found in bucket '{1}'")]
    IndexNotFound(String, String),
    #[error("No policy found for bucket '{0}'")]
    PolicyNotFound(String),
}

/// Per-account, per-region state for S3 Vectors.
#[derive(Debug, Default)]
pub struct S3VectorsState {
    /// Vector buckets keyed by name.
    pub buckets: HashMap<String, VectorBucket>,
    /// Indexes keyed by (bucket_name, index_name).
    pub indexes: HashMap<(String, String), Index>,
    /// Vectors keyed by (bucket_name, index_name, vector_key).
    pub vectors: HashMap<(String, String, String), Vector>,
    /// Policies keyed by bucket_name.
    pub policies: HashMap<String, VectorBucketPolicy>,
    /// Tags keyed by ARN.
    pub tags: HashMap<String, TagsMap>,
}

impl S3VectorsState {
    // -------------------------------------------------------------------------
    // VectorBucket helpers
    // -------------------------------------------------------------------------

    fn bucket_key<'a>(&self, name: Option<&'a str>, arn: Option<&'a str>) -> Option<&'a str> {
        if let Some(n) = name
            && !n.is_empty()
        {
            return Some(n);
        }
        if let Some(a) = arn {
            // Extract bucket name from ARN: arn:aws:s3vectors:region:acct:bucket/name
            return a.split('/').next_back().filter(|s| !s.is_empty());
        }
        None
    }

    pub fn create_bucket(
        &mut self,
        name: String,
        account_id: &str,
        region: &str,
        creation_time: String,
        tags: Option<HashMap<String, String>>,
    ) -> Result<VectorBucket, S3VectorsError> {
        if self.buckets.contains_key(&name) {
            return Err(S3VectorsError::BucketAlreadyExists(name));
        }
        let arn = format!("arn:aws:s3vectors:{region}:{account_id}:bucket/{name}");
        let bucket = VectorBucket {
            name: name.clone(),
            arn: arn.clone(),
            creation_time,
        };
        self.buckets.insert(name, bucket.clone());
        if let Some(t) = tags
            && !t.is_empty()
        {
            self.tags.insert(arn, t);
        }
        Ok(bucket)
    }

    pub fn get_bucket(
        &self,
        name: Option<&str>,
        arn: Option<&str>,
    ) -> Result<&VectorBucket, S3VectorsError> {
        let key = self
            .bucket_key(name, arn)
            .ok_or(S3VectorsError::BucketIdentifierRequired)?;
        self.buckets
            .get(key)
            .ok_or_else(|| S3VectorsError::BucketNotFound(key.to_string()))
    }

    pub fn delete_bucket(
        &mut self,
        name: Option<&str>,
        arn: Option<&str>,
    ) -> Result<(), S3VectorsError> {
        let key = self
            .bucket_key(name, arn)
            .ok_or(S3VectorsError::BucketIdentifierRequired)?
            .to_string();
        let has_indexes = self.indexes.keys().any(|(bname, _)| bname == &key);
        if has_indexes {
            return Err(S3VectorsError::BucketNotEmpty(key));
        }
        let bucket = self
            .buckets
            .remove(&key)
            .ok_or_else(|| S3VectorsError::BucketNotFound(key.clone()))?;
        self.tags.remove(&bucket.arn);
        self.policies.remove(&key);
        Ok(())
    }

    pub fn list_buckets(&self, prefix: Option<&str>) -> Vec<&VectorBucket> {
        let mut buckets: Vec<&VectorBucket> = self
            .buckets
            .values()
            .filter(|b| prefix.is_none_or(|p| b.name.starts_with(p)))
            .collect();
        buckets.sort_by(|a, b| a.name.cmp(&b.name));
        buckets
    }

    // -------------------------------------------------------------------------
    // Index helpers
    // -------------------------------------------------------------------------

    fn index_key<'a>(
        &self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        index_name: Option<&'a str>,
        index_arn: Option<&'a str>,
    ) -> Result<(String, String), S3VectorsError> {
        let bkey = if let Some(n) = bucket_name.filter(|s| !s.is_empty()) {
            n.to_string()
        } else if let Some(a) = bucket_arn.filter(|s| !s.is_empty()) {
            a.split('/').next_back().unwrap_or("").to_string()
        } else if let Some(a) = index_arn.filter(|s| !s.is_empty()) {
            // arn:aws:s3vectors:region:acct:bucket/bname/index/iname
            let parts: Vec<&str> = a.split('/').collect();
            if parts.len() >= 2 {
                parts[parts.len() - 3].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let ikey = if let Some(n) = index_name.filter(|s| !s.is_empty()) {
            n.to_string()
        } else if let Some(a) = index_arn.filter(|s| !s.is_empty()) {
            a.split('/').next_back().unwrap_or("").to_string()
        } else {
            String::new()
        };

        if bkey.is_empty() || ikey.is_empty() {
            return Err(S3VectorsError::IndexIdentifierRequired);
        }
        Ok((bkey, ikey))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_index(
        &mut self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        index_name: String,
        data_type: String,
        dimension: i32,
        distance_metric: String,
        non_filterable_metadata_keys: Vec<String>,
        account_id: &str,
        region: &str,
        creation_time: String,
        tags: Option<HashMap<String, String>>,
    ) -> Result<Index, S3VectorsError> {
        // Resolve bucket name
        let bname = if let Some(n) = bucket_name.filter(|s| !s.is_empty()) {
            n.to_string()
        } else if let Some(a) = bucket_arn.filter(|s| !s.is_empty()) {
            a.split('/').next_back().unwrap_or("").to_string()
        } else {
            return Err(S3VectorsError::BucketIdentifierRequired);
        };

        if !self.buckets.contains_key(&bname) {
            return Err(S3VectorsError::BucketNotFound(bname));
        }

        let ikey = (bname.clone(), index_name.clone());
        if self.indexes.contains_key(&ikey) {
            return Err(S3VectorsError::IndexAlreadyExists(index_name, bname));
        }

        let arn =
            format!("arn:aws:s3vectors:{region}:{account_id}:bucket/{bname}/index/{index_name}");
        let index = Index {
            bucket_name: bname.clone(),
            name: index_name.clone(),
            arn: arn.clone(),
            creation_time,
            data_type,
            dimension,
            distance_metric,
            non_filterable_metadata_keys,
        };
        self.indexes.insert(ikey, index.clone());
        if let Some(t) = tags
            && !t.is_empty()
        {
            self.tags.insert(arn, t);
        }
        Ok(index)
    }

    pub fn get_index(
        &self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        index_name: Option<&str>,
        index_arn: Option<&str>,
    ) -> Result<&Index, S3VectorsError> {
        let (bkey, ikey) = self.index_key(bucket_name, bucket_arn, index_name, index_arn)?;
        self.indexes
            .get(&(bkey.clone(), ikey.clone()))
            .ok_or(S3VectorsError::IndexNotFound(ikey, bkey))
    }

    pub fn delete_index(
        &mut self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        index_name: Option<&str>,
        index_arn: Option<&str>,
    ) -> Result<(), S3VectorsError> {
        let (bkey, ikey) = self.index_key(bucket_name, bucket_arn, index_name, index_arn)?;
        let index = self
            .indexes
            .remove(&(bkey.clone(), ikey.clone()))
            .ok_or_else(|| S3VectorsError::IndexNotFound(ikey.clone(), bkey.clone()))?;
        self.tags.remove(&index.arn);
        self.vectors.retain(|(b, i, _), _| b != &bkey || i != &ikey);
        Ok(())
    }

    pub fn list_indexes(
        &self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        prefix: Option<&str>,
    ) -> Result<Vec<&Index>, S3VectorsError> {
        let bname = if let Some(n) = bucket_name.filter(|s| !s.is_empty()) {
            n.to_string()
        } else if let Some(a) = bucket_arn.filter(|s| !s.is_empty()) {
            a.split('/').next_back().unwrap_or("").to_string()
        } else {
            return Err(S3VectorsError::BucketIdentifierRequired);
        };

        if !self.buckets.contains_key(&bname) {
            return Err(S3VectorsError::BucketNotFound(bname));
        }

        let mut indexes: Vec<&Index> = self
            .indexes
            .iter()
            .filter(|((b, i), _)| b == &bname && prefix.is_none_or(|p| i.starts_with(p)))
            .map(|(_, v)| v)
            .collect();
        indexes.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(indexes)
    }

    // -------------------------------------------------------------------------
    // Vector helpers
    // -------------------------------------------------------------------------

    pub fn put_vectors(
        &mut self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        index_name: Option<&str>,
        index_arn: Option<&str>,
        vectors: Vec<(String, Vec<f32>, Option<serde_json::Value>)>,
    ) -> Result<(), S3VectorsError> {
        let (bkey, ikey) = self.index_key(bucket_name, bucket_arn, index_name, index_arn)?;
        if !self.indexes.contains_key(&(bkey.clone(), ikey.clone())) {
            return Err(S3VectorsError::IndexNotFound(ikey, bkey));
        }
        for (key, data, metadata) in vectors {
            self.vectors.insert(
                (bkey.clone(), ikey.clone(), key.clone()),
                Vector {
                    key,
                    data,
                    metadata,
                },
            );
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub fn get_vectors(
        &self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        index_name: Option<&str>,
        index_arn: Option<&str>,
        keys: &[String],
        return_data: bool,
        return_metadata: bool,
    ) -> Result<Vec<(String, Option<Vec<f32>>, Option<serde_json::Value>)>, S3VectorsError> {
        let (bkey, ikey) = self.index_key(bucket_name, bucket_arn, index_name, index_arn)?;
        if !self.indexes.contains_key(&(bkey.clone(), ikey.clone())) {
            return Err(S3VectorsError::IndexNotFound(ikey, bkey));
        }
        let mut result = Vec::new();
        for key in keys {
            let vkey = (bkey.clone(), ikey.clone(), key.clone());
            if let Some(v) = self.vectors.get(&vkey) {
                let data = if return_data {
                    Some(v.data.clone())
                } else {
                    None
                };
                let metadata = if return_metadata {
                    v.metadata.clone()
                } else {
                    None
                };
                result.push((v.key.clone(), data, metadata));
            }
        }
        Ok(result)
    }

    pub fn delete_vectors(
        &mut self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        index_name: Option<&str>,
        index_arn: Option<&str>,
        keys: &[String],
    ) -> Result<(), S3VectorsError> {
        let (bkey, ikey) = self.index_key(bucket_name, bucket_arn, index_name, index_arn)?;
        if !self.indexes.contains_key(&(bkey.clone(), ikey.clone())) {
            return Err(S3VectorsError::IndexNotFound(ikey, bkey));
        }
        for key in keys {
            self.vectors
                .remove(&(bkey.clone(), ikey.clone(), key.clone()));
        }
        Ok(())
    }

    /// Single-page response — pagination not implemented; full result set
    /// returned in one call. The `max_results` argument truncates the result
    /// set, but no continuation token is returned even when truncation occurs.
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub fn list_vectors(
        &self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        index_name: Option<&str>,
        index_arn: Option<&str>,
        max_results: Option<i32>,
        return_data: bool,
        return_metadata: bool,
    ) -> Result<
        (
            Vec<(String, Option<Vec<f32>>, Option<serde_json::Value>)>,
            Option<String>,
        ),
        S3VectorsError,
    > {
        let (bkey, ikey) = self.index_key(bucket_name, bucket_arn, index_name, index_arn)?;
        if !self.indexes.contains_key(&(bkey.clone(), ikey.clone())) {
            return Err(S3VectorsError::IndexNotFound(ikey, bkey));
        }
        let mut vecs: Vec<&Vector> = self
            .vectors
            .iter()
            .filter(|((b, i, _), _)| b == &bkey && i == &ikey)
            .map(|(_, v)| v)
            .collect();
        vecs.sort_by(|a, b| a.key.cmp(&b.key));
        let limit = max_results.unwrap_or(100).min(1000) as usize;
        let vecs = &vecs[..vecs.len().min(limit)];
        let result: Vec<_> = vecs
            .iter()
            .map(|v| {
                let data = if return_data {
                    Some(v.data.clone())
                } else {
                    None
                };
                let metadata = if return_metadata {
                    v.metadata.clone()
                } else {
                    None
                };
                (v.key.clone(), data, metadata)
            })
            .collect();
        Ok((result, None))
    }

    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub fn query_vectors(
        &self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        index_name: Option<&str>,
        index_arn: Option<&str>,
        top_k: i32,
        query_vector: &[f32],
        return_metadata: bool,
        return_distance: bool,
    ) -> Result<(Vec<(String, f32, Option<serde_json::Value>)>, String), S3VectorsError> {
        let (bkey, ikey) = self.index_key(bucket_name, bucket_arn, index_name, index_arn)?;
        let index = self
            .indexes
            .get(&(bkey.clone(), ikey.clone()))
            .ok_or_else(|| S3VectorsError::IndexNotFound(ikey.clone(), bkey.clone()))?;
        let distance_metric = index.distance_metric.clone();

        let vecs: Vec<&Vector> = self
            .vectors
            .iter()
            .filter(|((b, i, _), _)| b == &bkey && i == &ikey)
            .map(|(_, v)| v)
            .collect();

        let mut scored: Vec<(String, f32, Option<serde_json::Value>)> = vecs
            .iter()
            .map(|v| {
                let dist = compute_distance(query_vector, &v.data, &distance_metric);
                let metadata = if return_metadata {
                    v.metadata.clone()
                } else {
                    None
                };
                (v.key.clone(), dist, metadata)
            })
            .collect();

        // Sort by distance ascending
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k as usize);

        if !return_distance {
            scored.iter_mut().for_each(|s| s.1 = 0.0);
        }

        Ok((scored, distance_metric))
    }

    // -------------------------------------------------------------------------
    // Policy helpers
    // -------------------------------------------------------------------------

    pub fn get_bucket_policy(
        &self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
    ) -> Result<&VectorBucketPolicy, S3VectorsError> {
        let key = self
            .bucket_key(bucket_name, bucket_arn)
            .ok_or(S3VectorsError::BucketIdentifierRequired)?
            .to_string();
        if !self.buckets.contains_key(&key) {
            return Err(S3VectorsError::BucketNotFound(key));
        }
        self.policies
            .get(&key)
            .ok_or_else(|| S3VectorsError::PolicyNotFound(key.clone()))
    }

    pub fn put_bucket_policy(
        &mut self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
        policy: String,
    ) -> Result<(), S3VectorsError> {
        let key = self
            .bucket_key(bucket_name, bucket_arn)
            .ok_or(S3VectorsError::BucketIdentifierRequired)?
            .to_string();
        if !self.buckets.contains_key(&key) {
            return Err(S3VectorsError::BucketNotFound(key));
        }
        self.policies.insert(key, VectorBucketPolicy { policy });
        Ok(())
    }

    pub fn delete_bucket_policy(
        &mut self,
        bucket_name: Option<&str>,
        bucket_arn: Option<&str>,
    ) -> Result<(), S3VectorsError> {
        let key = self
            .bucket_key(bucket_name, bucket_arn)
            .ok_or(S3VectorsError::BucketIdentifierRequired)?
            .to_string();
        if !self.buckets.contains_key(&key) {
            return Err(S3VectorsError::BucketNotFound(key));
        }
        self.policies.remove(&key);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Tag helpers
    // -------------------------------------------------------------------------

    pub fn list_tags(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        self.tags.entry(arn.to_string()).or_default().extend(tags);
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) {
        if let Some(t) = self.tags.get_mut(arn) {
            for k in keys {
                t.remove(k);
            }
        }
    }
}

fn compute_distance(a: &[f32], b: &[f32], metric: &str) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return f32::MAX;
    }
    match metric {
        "COSINE" => {
            let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
            let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
            let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
            if na == 0.0 || nb == 0.0 {
                return 1.0;
            }
            1.0 - dot / (na * nb)
        }
        _ => {
            // EUCLIDEAN
            a.iter()
                .zip(b.iter())
                .map(|(x, y)| (x - y).powi(2))
                .sum::<f32>()
                .sqrt()
        }
    }
}
