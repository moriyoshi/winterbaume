//! Serde-compatible view types for Rekognition state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::RekognitionService;
use crate::state::RekognitionState;
use crate::types::{Collection, VideoJob, VideoJobType};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RekognitionStateView {
    #[serde(default)]
    pub collections: HashMap<String, CollectionView>,
    #[serde(default)]
    pub video_jobs: HashMap<String, VideoJobView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionView {
    pub collection_id: String,
    pub collection_arn: String,
    pub face_count: u64,
    pub face_model_version: String,
    pub creation_timestamp: f64,
    pub user_count: u64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoJobView {
    pub job_id: String,
    pub job_tag: Option<String>,
    pub job_type: String,
    pub collection_id: Option<String>,
    #[serde(default)]
    pub seed: u64,
}

impl From<&RekognitionState> for RekognitionStateView {
    fn from(state: &RekognitionState) -> Self {
        RekognitionStateView {
            collections: state
                .collections
                .iter()
                .map(|(k, v)| (k.clone(), CollectionView::from(v)))
                .collect(),
            video_jobs: state
                .video_jobs
                .iter()
                .map(|(k, v)| (k.clone(), VideoJobView::from(v)))
                .collect(),
        }
    }
}

impl From<&Collection> for CollectionView {
    fn from(c: &Collection) -> Self {
        CollectionView {
            collection_id: c.collection_id.clone(),
            collection_arn: c.collection_arn.clone(),
            face_count: c.face_count,
            face_model_version: c.face_model_version.clone(),
            creation_timestamp: c.creation_timestamp,
            user_count: c.user_count,
            tags: c.tags.clone(),
        }
    }
}

impl From<&VideoJob> for VideoJobView {
    fn from(j: &VideoJob) -> Self {
        VideoJobView {
            job_id: j.job_id.clone(),
            job_tag: j.job_tag.clone(),
            job_type: match j.job_type {
                VideoJobType::FaceSearch => "FaceSearch".to_string(),
                VideoJobType::TextDetection => "TextDetection".to_string(),
            },
            collection_id: j.collection_id.clone(),
            seed: j.seed,
        }
    }
}

impl From<RekognitionStateView> for RekognitionState {
    fn from(view: RekognitionStateView) -> Self {
        RekognitionState {
            collections: view
                .collections
                .into_iter()
                .map(|(k, v)| (k, Collection::from(v)))
                .collect(),
            video_jobs: view
                .video_jobs
                .into_iter()
                .map(|(k, v)| (k, VideoJob::from(v)))
                .collect(),
        }
    }
}

impl From<CollectionView> for Collection {
    fn from(v: CollectionView) -> Self {
        Collection {
            collection_id: v.collection_id,
            collection_arn: v.collection_arn,
            face_count: v.face_count,
            face_model_version: v.face_model_version,
            creation_timestamp: v.creation_timestamp,
            user_count: v.user_count,
            tags: v.tags,
        }
    }
}

impl From<VideoJobView> for VideoJob {
    fn from(v: VideoJobView) -> Self {
        VideoJob {
            job_id: v.job_id,
            job_tag: v.job_tag,
            job_type: match v.job_type.as_str() {
                "TextDetection" => VideoJobType::TextDetection,
                _ => VideoJobType::FaceSearch,
            },
            collection_id: v.collection_id,
            seed: v.seed,
        }
    }
}

impl StatefulService for RekognitionService {
    type StateView = RekognitionStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        RekognitionStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = RekognitionState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.collections {
                guard.collections.insert(k, Collection::from(v));
            }
            for (k, v) in view.video_jobs {
                guard.video_jobs.insert(k, VideoJob::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
