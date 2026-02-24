use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CodeGuruReviewerService;
use crate::state::CodeGuruReviewerState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeGuruReviewerStateView {
    #[serde(default)]
    pub associations: HashMap<String, Value>,
    #[serde(default)]
    pub code_reviews: HashMap<String, Value>,
    #[serde(default)]
    pub recommendations: HashMap<String, Vec<Value>>,
    #[serde(default)]
    pub feedback: HashMap<String, Value>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

impl From<&CodeGuruReviewerState> for CodeGuruReviewerStateView {
    fn from(s: &CodeGuruReviewerState) -> Self {
        let feedback = s
            .feedback
            .iter()
            .map(|((arn, rec, user), v)| (format!("{arn}|{rec}|{user}"), v.clone()))
            .collect();
        Self {
            associations: s.associations.clone(),
            code_reviews: s.code_reviews.clone(),
            recommendations: s.recommendations.clone(),
            feedback,
            tags: s.tags.clone(),
        }
    }
}

impl From<CodeGuruReviewerStateView> for CodeGuruReviewerState {
    fn from(v: CodeGuruReviewerStateView) -> Self {
        let feedback = v
            .feedback
            .into_iter()
            .filter_map(|(k, val)| {
                let parts: Vec<&str> = k.splitn(3, '|').collect();
                if parts.len() == 3 {
                    Some((
                        (
                            parts[0].to_string(),
                            parts[1].to_string(),
                            parts[2].to_string(),
                        ),
                        val,
                    ))
                } else {
                    None
                }
            })
            .collect();
        Self {
            associations: v.associations,
            code_reviews: v.code_reviews,
            recommendations: v.recommendations,
            feedback,
            tags: v.tags,
        }
    }
}

impl StatefulService for CodeGuruReviewerService {
    type StateView = CodeGuruReviewerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CodeGuruReviewerStateView::from(&*guard)
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
            *guard = CodeGuruReviewerState::from(view);
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
        let new_state = CodeGuruReviewerState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in new_state.associations {
                guard.associations.insert(k, v);
            }
            for (k, v) in new_state.code_reviews {
                guard.code_reviews.insert(k, v);
            }
            for (k, v) in new_state.recommendations {
                guard.recommendations.insert(k, v);
            }
            for (k, v) in new_state.feedback {
                guard.feedback.insert(k, v);
            }
            for (k, v) in new_state.tags {
                guard.tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
