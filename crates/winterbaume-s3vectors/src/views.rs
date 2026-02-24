use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::S3VectorsService;
use crate::state::S3VectorsState;
use crate::types::{Index, TagsMap, Vector, VectorBucket, VectorBucketPolicy};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct S3VectorsStateView {
    #[serde(default)]
    pub buckets: HashMap<String, VectorBucketView>,
    #[serde(default)]
    pub indexes: HashMap<String, IndexView>,
    #[serde(default)]
    pub vectors: HashMap<String, VectorView>,
    #[serde(default)]
    pub policies: HashMap<String, String>,
    #[serde(default)]
    pub tags: HashMap<String, TagsMap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorBucketView {
    pub name: String,
    pub arn: String,
    pub creation_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexView {
    pub bucket_name: String,
    pub name: String,
    pub arn: String,
    pub creation_time: String,
    pub data_type: String,
    pub dimension: i32,
    pub distance_metric: String,
    #[serde(default)]
    pub non_filterable_metadata_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorView {
    pub bucket_name: String,
    pub index_name: String,
    pub key: String,
    pub data: Vec<f32>,
    pub metadata: Option<serde_json::Value>,
}

impl From<&VectorBucket> for VectorBucketView {
    fn from(b: &VectorBucket) -> Self {
        Self {
            name: b.name.clone(),
            arn: b.arn.clone(),
            creation_time: b.creation_time.clone(),
        }
    }
}

impl From<VectorBucketView> for VectorBucket {
    fn from(v: VectorBucketView) -> Self {
        Self {
            name: v.name,
            arn: v.arn,
            creation_time: v.creation_time,
        }
    }
}

impl From<&Index> for IndexView {
    fn from(i: &Index) -> Self {
        Self {
            bucket_name: i.bucket_name.clone(),
            name: i.name.clone(),
            arn: i.arn.clone(),
            creation_time: i.creation_time.clone(),
            data_type: i.data_type.clone(),
            dimension: i.dimension,
            distance_metric: i.distance_metric.clone(),
            non_filterable_metadata_keys: i.non_filterable_metadata_keys.clone(),
        }
    }
}

impl From<IndexView> for Index {
    fn from(v: IndexView) -> Self {
        Self {
            bucket_name: v.bucket_name,
            name: v.name,
            arn: v.arn,
            creation_time: v.creation_time,
            data_type: v.data_type,
            dimension: v.dimension,
            distance_metric: v.distance_metric,
            non_filterable_metadata_keys: v.non_filterable_metadata_keys,
        }
    }
}

impl From<&S3VectorsState> for S3VectorsStateView {
    fn from(state: &S3VectorsState) -> Self {
        Self {
            buckets: state
                .buckets
                .iter()
                .map(|(k, v)| (k.clone(), VectorBucketView::from(v)))
                .collect(),
            indexes: state
                .indexes
                .iter()
                .map(|((b, i), v)| (format!("{b}/{i}"), IndexView::from(v)))
                .collect(),
            vectors: state
                .vectors
                .iter()
                .map(|((b, i, k), v)| {
                    (
                        format!("{b}/{i}/{k}"),
                        VectorView {
                            bucket_name: b.clone(),
                            index_name: i.clone(),
                            key: k.clone(),
                            data: v.data.clone(),
                            metadata: v.metadata.clone(),
                        },
                    )
                })
                .collect(),
            policies: state
                .policies
                .iter()
                .map(|(k, v)| (k.clone(), v.policy.clone()))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<S3VectorsStateView> for S3VectorsState {
    fn from(view: S3VectorsStateView) -> Self {
        let mut state = S3VectorsState::default();
        for (_, b) in view.buckets {
            state.buckets.insert(b.name.clone(), VectorBucket::from(b));
        }
        for (_, i) in view.indexes {
            let key = (i.bucket_name.clone(), i.name.clone());
            state.indexes.insert(key, Index::from(i));
        }
        for (_, v) in view.vectors {
            let key = (v.bucket_name.clone(), v.index_name.clone(), v.key.clone());
            state.vectors.insert(
                key,
                Vector {
                    key: v.key,
                    data: v.data,
                    metadata: v.metadata,
                },
            );
        }
        for (k, p) in view.policies {
            state.policies.insert(k, VectorBucketPolicy { policy: p });
        }
        state.tags = view.tags;
        state
    }
}

impl StatefulService for S3VectorsService {
    type StateView = S3VectorsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        S3VectorsStateView::from(&*guard)
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
            *guard = S3VectorsState::from(view);
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
            for (_, b) in view.buckets {
                guard.buckets.insert(b.name.clone(), VectorBucket::from(b));
            }
            for (_, i) in view.indexes {
                let key = (i.bucket_name.clone(), i.name.clone());
                guard.indexes.insert(key, Index::from(i));
            }
            for (_, v) in view.vectors {
                let key = (v.bucket_name.clone(), v.index_name.clone(), v.key.clone());
                guard.vectors.insert(
                    key,
                    Vector {
                        key: v.key,
                        data: v.data,
                        metadata: v.metadata,
                    },
                );
            }
            for (k, p) in view.policies {
                guard.policies.insert(k, VectorBucketPolicy { policy: p });
            }
            guard.tags.extend(view.tags);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
