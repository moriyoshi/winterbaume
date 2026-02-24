use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct RekognitionState {
    pub collections: HashMap<String, Collection>,
    pub video_jobs: HashMap<String, VideoJob>,
}

#[derive(Debug, Error)]
pub enum RekognitionError {
    #[error("A collection with the specified ID already exists.")]
    ResourceAlreadyExists { collection_id: String },

    #[error("The collection id: {collection_id} does not exist")]
    CollectionNotFound { collection_id: String },

    #[error("The job id: {job_id} does not exist")]
    VideoJobNotFound { job_id: String },
}

impl RekognitionState {
    pub fn create_collection(
        &mut self,
        collection_id: &str,
        tags: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&Collection, RekognitionError> {
        if self.collections.contains_key(collection_id) {
            return Err(RekognitionError::ResourceAlreadyExists {
                collection_id: collection_id.to_string(),
            });
        }

        let arn = format!("arn:aws:rekognition:{region}:{account_id}:collection/{collection_id}");
        let now = chrono::Utc::now().timestamp_millis() as f64 / 1000.0;

        let collection = Collection {
            collection_id: collection_id.to_string(),
            collection_arn: arn,
            face_count: 0,
            face_model_version: "6.0".to_string(),
            creation_timestamp: now,
            user_count: 0,
            tags,
        };

        self.collections
            .insert(collection_id.to_string(), collection);
        Ok(self.collections.get(collection_id).unwrap())
    }

    pub fn describe_collection(
        &self,
        collection_id: &str,
    ) -> Result<&Collection, RekognitionError> {
        match self.collections.get(collection_id) {
            Some(c) => Ok(c),
            None => Err(RekognitionError::CollectionNotFound {
                collection_id: collection_id.to_string(),
            }),
        }
    }

    pub fn delete_collection(&mut self, collection_id: &str) -> Result<(), RekognitionError> {
        match self.collections.remove(collection_id) {
            Some(_) => Ok(()),
            None => Err(RekognitionError::CollectionNotFound {
                collection_id: collection_id.to_string(),
            }),
        }
    }

    pub fn list_collections(
        &self,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> (Vec<&Collection>, Option<String>) {
        let mut ids: Vec<&String> = self.collections.keys().collect();
        ids.sort();

        let start_idx = if let Some(token) = next_token {
            ids.iter()
                .position(|id| id.as_str() > token)
                .unwrap_or(ids.len())
        } else {
            0
        };

        let limit = max_results.unwrap_or(4096);
        let end_idx = (start_idx + limit).min(ids.len());

        let result: Vec<&Collection> = ids[start_idx..end_idx]
            .iter()
            .filter_map(|id| self.collections.get(id.as_str()))
            .collect();

        let next = if end_idx < ids.len() {
            ids.get(end_idx - 1).map(|id| id.to_string())
        } else {
            None
        };

        (result, next)
    }

    pub fn start_video_job(
        &mut self,
        job_type: VideoJobType,
        job_tag: Option<String>,
        collection_id: Option<String>,
    ) -> String {
        use std::hash::{Hash, Hasher};
        let job_id = uuid::Uuid::new_v4().to_string();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        job_id.hash(&mut hasher);
        if let Some(ref cid) = collection_id {
            cid.hash(&mut hasher);
        }
        let seed = hasher.finish();
        let job = VideoJob {
            job_id: job_id.clone(),
            job_tag,
            job_type,
            collection_id,
            seed,
        };
        self.video_jobs.insert(job_id.clone(), job);
        job_id
    }

    pub fn get_video_job(&self, job_id: &str) -> Result<&VideoJob, RekognitionError> {
        match self.video_jobs.get(job_id) {
            Some(job) => Ok(job),
            None => Err(RekognitionError::VideoJobNotFound {
                job_id: job_id.to_string(),
            }),
        }
    }
}
