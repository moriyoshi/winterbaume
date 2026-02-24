use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::model;
use crate::state::{PersonalizeError, PersonalizeState};
use crate::views::PersonalizeStateView;
use crate::wire;

pub struct PersonalizeService {
    pub(crate) state: Arc<BackendState<PersonalizeState>>,
    pub(crate) notifier: StateChangeNotifier<PersonalizeStateView>,
}

impl PersonalizeService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PersonalizeService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PersonalizeService {
    fn service_name(&self) -> &str {
        "personalize"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://personalize\..*\.amazonaws\.com",
            r"https?://personalize\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl PersonalizeService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return personalize_error_response(&PersonalizeError::MissingAction);
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return personalize_error_response(&PersonalizeError::SerializationException);
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "CreateDatasetGroup" => {
                self.handle_create_dataset_group(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeDatasetGroup" => self.handle_describe_dataset_group(&state, body_bytes).await,
            "DeleteDatasetGroup" => self.handle_delete_dataset_group(&state, body_bytes).await,
            "ListDatasetGroups" => self.handle_list_dataset_groups(&state).await,
            "CreateBatchInferenceJob" => {
                self.handle_create_batch_inference_job(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeBatchInferenceJob" => {
                self.handle_describe_batch_inference_job(&state, body_bytes)
                    .await
            }
            "ListBatchInferenceJobs" => {
                self.handle_list_batch_inference_jobs(&state, body_bytes)
                    .await
            }
            "CreateBatchSegmentJob" => {
                self.handle_create_batch_segment_job(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeBatchSegmentJob" => {
                self.handle_describe_batch_segment_job(&state, body_bytes)
                    .await
            }
            "ListBatchSegmentJobs" => {
                self.handle_list_batch_segment_jobs(&state, body_bytes)
                    .await
            }
            "CreateCampaign" => {
                self.handle_create_campaign(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeCampaign" => self.handle_describe_campaign(&state, body_bytes).await,
            "DeleteCampaign" => self.handle_delete_campaign(&state, body_bytes).await,
            "ListCampaigns" => self.handle_list_campaigns(&state, body_bytes).await,
            "UpdateCampaign" => self.handle_update_campaign(&state, body_bytes).await,
            "CreateDataDeletionJob" => {
                self.handle_create_data_deletion_job(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeDataDeletionJob" => {
                self.handle_describe_data_deletion_job(&state, body_bytes)
                    .await
            }
            "ListDataDeletionJobs" => {
                self.handle_list_data_deletion_jobs(&state, body_bytes)
                    .await
            }
            "CreateDataset" => {
                self.handle_create_dataset(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeDataset" => self.handle_describe_dataset(&state, body_bytes).await,
            "DeleteDataset" => self.handle_delete_dataset(&state, body_bytes).await,
            "ListDatasets" => self.handle_list_datasets(&state, body_bytes).await,
            "UpdateDataset" => self.handle_update_dataset(&state, body_bytes).await,
            "CreateDatasetExportJob" => {
                self.handle_create_dataset_export_job(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeDatasetExportJob" => {
                self.handle_describe_dataset_export_job(&state, body_bytes)
                    .await
            }
            "ListDatasetExportJobs" => {
                self.handle_list_dataset_export_jobs(&state, body_bytes)
                    .await
            }
            "CreateDatasetImportJob" => {
                self.handle_create_dataset_import_job(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeDatasetImportJob" => {
                self.handle_describe_dataset_import_job(&state, body_bytes)
                    .await
            }
            "ListDatasetImportJobs" => {
                self.handle_list_dataset_import_jobs(&state, body_bytes)
                    .await
            }
            "CreateEventTracker" => {
                self.handle_create_event_tracker(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeEventTracker" => self.handle_describe_event_tracker(&state, body_bytes).await,
            "DeleteEventTracker" => self.handle_delete_event_tracker(&state, body_bytes).await,
            "ListEventTrackers" => self.handle_list_event_trackers(&state, body_bytes).await,
            "CreateFilter" => {
                self.handle_create_filter(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeFilter" => self.handle_describe_filter(&state, body_bytes).await,
            "DeleteFilter" => self.handle_delete_filter(&state, body_bytes).await,
            "ListFilters" => self.handle_list_filters(&state, body_bytes).await,
            "CreateMetricAttribution" => {
                self.handle_create_metric_attribution(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeMetricAttribution" => {
                self.handle_describe_metric_attribution(&state, body_bytes)
                    .await
            }
            "DeleteMetricAttribution" => {
                self.handle_delete_metric_attribution(&state, body_bytes)
                    .await
            }
            "ListMetricAttributions" => {
                self.handle_list_metric_attributions(&state, body_bytes)
                    .await
            }
            "UpdateMetricAttribution" => {
                self.handle_update_metric_attribution(&state, body_bytes)
                    .await
            }
            "ListMetricAttributionMetrics" => {
                self.handle_list_metric_attribution_metrics(&state, body_bytes)
                    .await
            }
            "CreateRecommender" => {
                self.handle_create_recommender(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeRecommender" => self.handle_describe_recommender(&state, body_bytes).await,
            "DeleteRecommender" => self.handle_delete_recommender(&state, body_bytes).await,
            "ListRecommenders" => self.handle_list_recommenders(&state, body_bytes).await,
            "UpdateRecommender" => self.handle_update_recommender(&state, body_bytes).await,
            "StartRecommender" => self.handle_start_recommender(&state, body_bytes).await,
            "StopRecommender" => self.handle_stop_recommender(&state, body_bytes).await,
            "CreateSchema" => {
                self.handle_create_schema(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeSchema" => self.handle_describe_schema(&state, body_bytes).await,
            "DeleteSchema" => self.handle_delete_schema(&state, body_bytes).await,
            "ListSchemas" => self.handle_list_schemas(&state).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "CreateSolution" => {
                self.handle_create_solution(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeSolution" => self.handle_describe_solution(&state, body_bytes).await,
            "DeleteSolution" => self.handle_delete_solution(&state, body_bytes).await,
            "ListSolutions" => self.handle_list_solutions(&state, body_bytes).await,
            "UpdateSolution" => self.handle_update_solution(&state, body_bytes).await,
            "CreateSolutionVersion" => {
                self.handle_create_solution_version(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeSolutionVersion" => {
                self.handle_describe_solution_version(&state, body_bytes)
                    .await
            }
            "ListSolutionVersions" => self.handle_list_solution_versions(&state, body_bytes).await,
            "StopSolutionVersionCreation" => {
                self.handle_stop_solution_version_creation(&state, body_bytes)
                    .await
            }
            "GetSolutionMetrics" => self.handle_get_solution_metrics(&state, body_bytes).await,
            // Built-in catalogue stubs -- return empty/minimal data
            "DescribeAlgorithm" => self.handle_describe_algorithm(body_bytes).await,
            "DescribeFeatureTransformation" => {
                self.handle_describe_feature_transformation(body_bytes)
                    .await
            }
            "DescribeRecipe" => self.handle_describe_recipe(body_bytes).await,
            "ListRecipes" => self.handle_list_recipes().await,
            _ => personalize_error_response(&PersonalizeError::InvalidAction {
                action: action.to_string(),
            }),
        }
    }

    // ======================== Dataset Groups ========================

    async fn handle_create_dataset_group(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_dataset_group_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "name".to_string(),
            });
        }
        let name = input.name.as_str();
        let role_arn = input.role_arn.as_deref();
        let kms_key_arn = input.kms_key_arn.as_deref();
        let domain = input.domain.as_deref();

        let mut state = state.write().await;
        match state.create_dataset_group(name, role_arn, kms_key_arn, domain, region, account_id) {
            Ok(dg) => {
                let resp = model::CreateDatasetGroupResponse {
                    dataset_group_arn: Some(dg.dataset_group_arn.clone()),
                    domain: dg.domain.clone(),
                };
                wire::serialize_create_dataset_group_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_dataset_group(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_dataset_group_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.dataset_group_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetGroupArn".to_string(),
            });
        }
        let dataset_group_arn = input.dataset_group_arn.as_str();

        let state = state.read().await;
        match state.describe_dataset_group(dataset_group_arn) {
            Ok(dg) => {
                let resp = model::DescribeDatasetGroupResponse {
                    dataset_group: Some(dataset_group_to_model(dg)),
                };
                wire::serialize_describe_dataset_group_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_delete_dataset_group(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_dataset_group_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.dataset_group_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetGroupArn".to_string(),
            });
        }
        let dataset_group_arn = input.dataset_group_arn.as_str();

        let mut state = state.write().await;
        match state.delete_dataset_group(dataset_group_arn) {
            Ok(()) => wire::serialize_delete_dataset_group_response(),
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_dataset_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let groups = state.list_dataset_groups();
        let entries: Vec<model::DatasetGroupSummary> = groups
            .iter()
            .map(|dg| dataset_group_to_summary(dg))
            .collect();
        let resp = model::ListDatasetGroupsResponse {
            dataset_groups: Some(entries),
            next_token: None,
        };
        wire::serialize_list_dataset_groups_response(&resp)
    }

    // ======================== Schemas ========================

    async fn handle_create_schema(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_schema_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "name".to_string(),
            });
        }
        if input.schema.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "schema".to_string(),
            });
        }
        let name = input.name.as_str();
        let schema = input.schema.as_str();
        let domain = input.domain.as_deref();

        let mut state = state.write().await;
        match state.create_schema(name, schema, domain, region, account_id) {
            Ok(s) => {
                let resp = model::CreateSchemaResponse {
                    schema_arn: Some(s.schema_arn.clone()),
                };
                wire::serialize_create_schema_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_schema(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_schema_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.schema_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "schemaArn".to_string(),
            });
        }
        let schema_arn = input.schema_arn.as_str();
        let state = state.read().await;
        match state.describe_schema(schema_arn) {
            Ok(s) => {
                let resp = model::DescribeSchemaResponse {
                    schema: Some(schema_to_model(s)),
                };
                wire::serialize_describe_schema_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_delete_schema(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_schema_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.schema_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "schemaArn".to_string(),
            });
        }
        let schema_arn = input.schema_arn.as_str();
        let mut state = state.write().await;
        match state.delete_schema(schema_arn) {
            Ok(()) => wire::serialize_delete_schema_response(),
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_schemas(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let schemas = state.list_schemas();
        let entries: Vec<model::DatasetSchemaSummary> =
            schemas.iter().map(|s| schema_to_summary(s)).collect();
        let resp = model::ListSchemasResponse {
            schemas: Some(entries),
            next_token: None,
        };
        wire::serialize_list_schemas_response(&resp)
    }

    // ======================== Datasets ========================

    async fn handle_create_dataset(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_dataset_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "name".to_string(),
            });
        }
        if input.dataset_group_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetGroupArn".to_string(),
            });
        }
        if input.dataset_type.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetType".to_string(),
            });
        }
        if input.schema_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "schemaArn".to_string(),
            });
        }
        let name = input.name.as_str();
        let dataset_group_arn = input.dataset_group_arn.as_str();
        let dataset_type = input.dataset_type.as_str();
        let schema_arn = input.schema_arn.as_str();

        let mut state = state.write().await;
        match state.create_dataset(
            name,
            dataset_group_arn,
            dataset_type,
            schema_arn,
            region,
            account_id,
        ) {
            Ok(ds) => {
                let resp = model::CreateDatasetResponse {
                    dataset_arn: Some(ds.dataset_arn.clone()),
                };
                wire::serialize_create_dataset_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_dataset(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_dataset_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.dataset_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetArn".to_string(),
            });
        }
        let dataset_arn = input.dataset_arn.as_str();
        let state = state.read().await;
        match state.describe_dataset(dataset_arn) {
            Ok(ds) => {
                let resp = model::DescribeDatasetResponse {
                    dataset: Some(dataset_to_model(ds)),
                };
                wire::serialize_describe_dataset_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_delete_dataset(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_dataset_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.dataset_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetArn".to_string(),
            });
        }
        let dataset_arn = input.dataset_arn.as_str();
        let mut state = state.write().await;
        match state.delete_dataset(dataset_arn) {
            Ok(()) => wire::serialize_delete_dataset_response(),
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_datasets(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_datasets_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let dataset_group_arn = input.dataset_group_arn.as_deref();
        let state = state.read().await;
        let datasets = state.list_datasets(dataset_group_arn);
        let entries: Vec<model::DatasetSummary> =
            datasets.iter().map(|ds| dataset_to_summary(ds)).collect();
        let resp = model::ListDatasetsResponse {
            datasets: Some(entries),
            next_token: None,
        };
        wire::serialize_list_datasets_response(&resp)
    }

    async fn handle_update_dataset(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_dataset_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.dataset_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetArn".to_string(),
            });
        }
        if input.schema_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "schemaArn".to_string(),
            });
        }
        let dataset_arn = input.dataset_arn.as_str();
        let schema_arn = input.schema_arn.as_str();
        let mut state = state.write().await;
        match state.update_dataset(dataset_arn, schema_arn) {
            Ok(ds) => {
                let resp = model::UpdateDatasetResponse {
                    dataset_arn: Some(ds.dataset_arn.clone()),
                };
                wire::serialize_update_dataset_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    // ======================== Campaigns ========================

    async fn handle_create_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_campaign_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "name".to_string(),
            });
        }
        if input.solution_version_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "solutionVersionArn".to_string(),
            });
        }
        let name = input.name.as_str();
        let solution_version_arn = input.solution_version_arn.as_str();
        let min_provisioned_tps = input.min_provisioned_t_p_s;

        let mut state = state.write().await;
        match state.create_campaign(
            name,
            solution_version_arn,
            min_provisioned_tps,
            region,
            account_id,
        ) {
            Ok(c) => {
                let resp = model::CreateCampaignResponse {
                    campaign_arn: Some(c.campaign_arn.clone()),
                };
                wire::serialize_create_campaign_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_campaign_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.campaign_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "campaignArn".to_string(),
            });
        }
        let campaign_arn = input.campaign_arn.as_str();
        let state = state.read().await;
        match state.describe_campaign(campaign_arn) {
            Ok(c) => {
                let resp = model::DescribeCampaignResponse {
                    campaign: Some(campaign_to_model(c)),
                };
                wire::serialize_describe_campaign_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_delete_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_campaign_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.campaign_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "campaignArn".to_string(),
            });
        }
        let campaign_arn = input.campaign_arn.as_str();
        let mut state = state.write().await;
        match state.delete_campaign(campaign_arn) {
            Ok(()) => wire::serialize_delete_campaign_response(),
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_campaigns(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_campaigns_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let solution_arn = input.solution_arn.as_deref();
        let state = state.read().await;
        let campaigns = state.list_campaigns(solution_arn);
        let entries: Vec<model::CampaignSummary> =
            campaigns.iter().map(|c| campaign_to_summary(c)).collect();
        let resp = model::ListCampaignsResponse {
            campaigns: Some(entries),
            next_token: None,
        };
        wire::serialize_list_campaigns_response(&resp)
    }

    async fn handle_update_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_campaign_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.campaign_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "campaignArn".to_string(),
            });
        }
        let campaign_arn = input.campaign_arn.as_str();
        let solution_version_arn = input.solution_version_arn.as_deref();
        let min_provisioned_tps = input.min_provisioned_t_p_s;
        let mut state = state.write().await;
        match state.update_campaign(campaign_arn, solution_version_arn, min_provisioned_tps) {
            Ok(c) => {
                let resp = model::UpdateCampaignResponse {
                    campaign_arn: Some(c.campaign_arn.clone()),
                };
                wire::serialize_update_campaign_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    // ======================== Event Trackers ========================

    async fn handle_create_event_tracker(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_event_tracker_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "name".to_string(),
            });
        }
        if input.dataset_group_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetGroupArn".to_string(),
            });
        }
        let name = input.name.as_str();
        let dataset_group_arn = input.dataset_group_arn.as_str();

        let mut state = state.write().await;
        match state.create_event_tracker(name, dataset_group_arn, region, account_id) {
            Ok(et) => {
                let resp = model::CreateEventTrackerResponse {
                    event_tracker_arn: Some(et.event_tracker_arn.clone()),
                    tracking_id: Some(et.tracking_id.clone()),
                };
                wire::serialize_create_event_tracker_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_event_tracker(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_event_tracker_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.event_tracker_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "eventTrackerArn".to_string(),
            });
        }
        let event_tracker_arn = input.event_tracker_arn.as_str();
        let state = state.read().await;
        match state.describe_event_tracker(event_tracker_arn) {
            Ok(et) => {
                let resp = model::DescribeEventTrackerResponse {
                    event_tracker: Some(event_tracker_to_model(et)),
                };
                wire::serialize_describe_event_tracker_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_delete_event_tracker(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_event_tracker_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.event_tracker_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "eventTrackerArn".to_string(),
            });
        }
        let event_tracker_arn = input.event_tracker_arn.as_str();
        let mut state = state.write().await;
        match state.delete_event_tracker(event_tracker_arn) {
            Ok(()) => wire::serialize_delete_event_tracker_response(),
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_event_trackers(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_event_trackers_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let dataset_group_arn = input.dataset_group_arn.as_deref();
        let state = state.read().await;
        let trackers = state.list_event_trackers(dataset_group_arn);
        let entries: Vec<model::EventTrackerSummary> = trackers
            .iter()
            .map(|et| event_tracker_to_summary(et))
            .collect();
        let resp = model::ListEventTrackersResponse {
            event_trackers: Some(entries),
            next_token: None,
        };
        wire::serialize_list_event_trackers_response(&resp)
    }

    // ======================== Filters ========================

    async fn handle_create_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_filter_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "name".to_string(),
            });
        }
        if input.dataset_group_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetGroupArn".to_string(),
            });
        }
        if input.filter_expression.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "filterExpression".to_string(),
            });
        }
        let name = input.name.as_str();
        let dataset_group_arn = input.dataset_group_arn.as_str();
        let filter_expression = input.filter_expression.as_str();

        let mut state = state.write().await;
        match state.create_filter(
            name,
            dataset_group_arn,
            filter_expression,
            region,
            account_id,
        ) {
            Ok(f) => {
                let resp = model::CreateFilterResponse {
                    filter_arn: Some(f.filter_arn.clone()),
                };
                wire::serialize_create_filter_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_filter_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.filter_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "filterArn".to_string(),
            });
        }
        let filter_arn = input.filter_arn.as_str();
        let state = state.read().await;
        match state.describe_filter(filter_arn) {
            Ok(f) => {
                let resp = model::DescribeFilterResponse {
                    filter: Some(filter_to_model(f)),
                };
                wire::serialize_describe_filter_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_delete_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_filter_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.filter_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "filterArn".to_string(),
            });
        }
        let filter_arn = input.filter_arn.as_str();
        let mut state = state.write().await;
        match state.delete_filter(filter_arn) {
            Ok(()) => wire::serialize_delete_filter_response(),
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_filters(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_filters_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let dataset_group_arn = input.dataset_group_arn.as_deref();
        let state = state.read().await;
        let filters = state.list_filters(dataset_group_arn);
        let entries: Vec<model::FilterSummary> =
            filters.iter().map(|f| filter_to_summary(f)).collect();
        let resp = model::ListFiltersResponse {
            filters: Some(entries),
            next_token: None,
        };
        wire::serialize_list_filters_response(&resp)
    }

    // ======================== Batch Inference Jobs ========================

    async fn handle_create_batch_inference_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_batch_inference_job_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.job_name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "jobName".to_string(),
            });
        }
        if input.solution_version_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "solutionVersionArn".to_string(),
            });
        }
        if input.role_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "roleArn".to_string(),
            });
        }
        let job_name = input.job_name.as_str();
        let solution_version_arn = input.solution_version_arn.as_str();
        let role_arn = input.role_arn.as_str();
        let filter_arn = input.filter_arn.as_deref();
        let num_results = input.num_results;
        let job_input_s3 = input.job_input.s3_data_source.path.as_str();
        let job_output_s3 = input.job_output.s3_data_destination.path.as_str();
        let batch_inference_job_mode = input.batch_inference_job_mode.as_deref();

        let mut state = state.write().await;
        match state.create_batch_inference_job(
            job_name,
            solution_version_arn,
            filter_arn,
            role_arn,
            num_results,
            job_input_s3,
            job_output_s3,
            batch_inference_job_mode,
            region,
            account_id,
        ) {
            Ok(j) => {
                let resp = model::CreateBatchInferenceJobResponse {
                    batch_inference_job_arn: Some(j.batch_inference_job_arn.clone()),
                };
                wire::serialize_create_batch_inference_job_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_batch_inference_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_batch_inference_job_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.batch_inference_job_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "batchInferenceJobArn".to_string(),
            });
        }
        let arn = input.batch_inference_job_arn.as_str();
        let state = state.read().await;
        match state.describe_batch_inference_job(arn) {
            Ok(j) => {
                let resp = model::DescribeBatchInferenceJobResponse {
                    batch_inference_job: Some(batch_inference_job_to_model(j)),
                };
                wire::serialize_describe_batch_inference_job_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_batch_inference_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_batch_inference_jobs_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let solution_version_arn = input.solution_version_arn.as_deref();
        let state = state.read().await;
        let jobs = state.list_batch_inference_jobs(solution_version_arn);
        let entries: Vec<model::BatchInferenceJobSummary> = jobs
            .iter()
            .map(|j| batch_inference_job_to_summary(j))
            .collect();
        let resp = model::ListBatchInferenceJobsResponse {
            batch_inference_jobs: Some(entries),
            next_token: None,
        };
        wire::serialize_list_batch_inference_jobs_response(&resp)
    }

    // ======================== Batch Segment Jobs ========================

    async fn handle_create_batch_segment_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_batch_segment_job_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.job_name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "jobName".to_string(),
            });
        }
        if input.solution_version_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "solutionVersionArn".to_string(),
            });
        }
        if input.role_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "roleArn".to_string(),
            });
        }
        let job_name = input.job_name.as_str();
        let solution_version_arn = input.solution_version_arn.as_str();
        let role_arn = input.role_arn.as_str();
        let filter_arn = input.filter_arn.as_deref();
        let num_results = input.num_results;
        let job_input_s3 = input.job_input.s3_data_source.path.as_str();
        let job_output_s3 = input.job_output.s3_data_destination.path.as_str();

        let mut state = state.write().await;
        match state.create_batch_segment_job(
            job_name,
            solution_version_arn,
            filter_arn,
            role_arn,
            num_results,
            job_input_s3,
            job_output_s3,
            region,
            account_id,
        ) {
            Ok(j) => {
                let resp = model::CreateBatchSegmentJobResponse {
                    batch_segment_job_arn: Some(j.batch_segment_job_arn.clone()),
                };
                wire::serialize_create_batch_segment_job_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_batch_segment_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_batch_segment_job_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.batch_segment_job_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "batchSegmentJobArn".to_string(),
            });
        }
        let arn = input.batch_segment_job_arn.as_str();
        let state = state.read().await;
        match state.describe_batch_segment_job(arn) {
            Ok(j) => {
                let resp = model::DescribeBatchSegmentJobResponse {
                    batch_segment_job: Some(batch_segment_job_to_model(j)),
                };
                wire::serialize_describe_batch_segment_job_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_batch_segment_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_batch_segment_jobs_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let solution_version_arn = input.solution_version_arn.as_deref();
        let state = state.read().await;
        let jobs = state.list_batch_segment_jobs(solution_version_arn);
        let entries: Vec<model::BatchSegmentJobSummary> = jobs
            .iter()
            .map(|j| batch_segment_job_to_summary(j))
            .collect();
        let resp = model::ListBatchSegmentJobsResponse {
            batch_segment_jobs: Some(entries),
            next_token: None,
        };
        wire::serialize_list_batch_segment_jobs_response(&resp)
    }

    // ======================== Data Deletion Jobs ========================

    async fn handle_create_data_deletion_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_data_deletion_job_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.job_name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "jobName".to_string(),
            });
        }
        if input.dataset_group_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetGroupArn".to_string(),
            });
        }
        if input.role_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "roleArn".to_string(),
            });
        }
        let job_name = input.job_name.as_str();
        let dataset_group_arn = input.dataset_group_arn.as_str();
        let role_arn = input.role_arn.as_str();
        let data_source_location = input.data_source.data_location.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.create_data_deletion_job(
            job_name,
            dataset_group_arn,
            data_source_location,
            role_arn,
            region,
            account_id,
        ) {
            Ok(j) => {
                let resp = model::CreateDataDeletionJobResponse {
                    data_deletion_job_arn: Some(j.data_deletion_job_arn.clone()),
                };
                wire::serialize_create_data_deletion_job_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_data_deletion_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_data_deletion_job_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.data_deletion_job_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "dataDeletionJobArn".to_string(),
            });
        }
        let arn = input.data_deletion_job_arn.as_str();
        let state = state.read().await;
        match state.describe_data_deletion_job(arn) {
            Ok(j) => {
                let resp = model::DescribeDataDeletionJobResponse {
                    data_deletion_job: Some(data_deletion_job_to_model(j)),
                };
                wire::serialize_describe_data_deletion_job_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_data_deletion_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_data_deletion_jobs_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let dataset_group_arn = input.dataset_group_arn.as_deref();
        let state = state.read().await;
        let jobs = state.list_data_deletion_jobs(dataset_group_arn);
        let entries: Vec<model::DataDeletionJobSummary> = jobs
            .iter()
            .map(|j| data_deletion_job_to_summary(j))
            .collect();
        let resp = model::ListDataDeletionJobsResponse {
            data_deletion_jobs: Some(entries),
            next_token: None,
        };
        wire::serialize_list_data_deletion_jobs_response(&resp)
    }

    // ======================== Dataset Export Jobs ========================

    async fn handle_create_dataset_export_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_dataset_export_job_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.job_name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "jobName".to_string(),
            });
        }
        if input.dataset_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetArn".to_string(),
            });
        }
        if input.role_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "roleArn".to_string(),
            });
        }
        let job_name = input.job_name.as_str();
        let dataset_arn = input.dataset_arn.as_str();
        let role_arn = input.role_arn.as_str();
        let ingestion_mode = input.ingestion_mode.as_deref();
        let job_output_s3 = input.job_output.s3_data_destination.path.as_str();

        let mut state = state.write().await;
        match state.create_dataset_export_job(
            job_name,
            dataset_arn,
            role_arn,
            ingestion_mode,
            job_output_s3,
            region,
            account_id,
        ) {
            Ok(j) => {
                let resp = model::CreateDatasetExportJobResponse {
                    dataset_export_job_arn: Some(j.dataset_export_job_arn.clone()),
                };
                wire::serialize_create_dataset_export_job_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_dataset_export_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_dataset_export_job_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.dataset_export_job_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetExportJobArn".to_string(),
            });
        }
        let arn = input.dataset_export_job_arn.as_str();
        let state = state.read().await;
        match state.describe_dataset_export_job(arn) {
            Ok(j) => {
                let resp = model::DescribeDatasetExportJobResponse {
                    dataset_export_job: Some(dataset_export_job_to_model(j)),
                };
                wire::serialize_describe_dataset_export_job_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_dataset_export_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_dataset_export_jobs_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let dataset_arn = input.dataset_arn.as_deref();
        let state = state.read().await;
        let jobs = state.list_dataset_export_jobs(dataset_arn);
        let entries: Vec<model::DatasetExportJobSummary> = jobs
            .iter()
            .map(|j| dataset_export_job_to_summary(j))
            .collect();
        let resp = model::ListDatasetExportJobsResponse {
            dataset_export_jobs: Some(entries),
            next_token: None,
        };
        wire::serialize_list_dataset_export_jobs_response(&resp)
    }

    // ======================== Dataset Import Jobs ========================

    async fn handle_create_dataset_import_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_dataset_import_job_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.job_name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "jobName".to_string(),
            });
        }
        if input.dataset_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetArn".to_string(),
            });
        }
        let job_name = input.job_name.as_str();
        let dataset_arn = input.dataset_arn.as_str();
        let role_arn = input.role_arn.as_deref();
        let import_mode = input.import_mode.as_deref();
        let data_source_location = input.data_source.data_location.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.create_dataset_import_job(
            job_name,
            dataset_arn,
            data_source_location,
            role_arn,
            import_mode,
            region,
            account_id,
        ) {
            Ok(j) => {
                let resp = model::CreateDatasetImportJobResponse {
                    dataset_import_job_arn: Some(j.dataset_import_job_arn.clone()),
                };
                wire::serialize_create_dataset_import_job_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_dataset_import_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_dataset_import_job_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.dataset_import_job_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetImportJobArn".to_string(),
            });
        }
        let arn = input.dataset_import_job_arn.as_str();
        let state = state.read().await;
        match state.describe_dataset_import_job(arn) {
            Ok(j) => {
                let resp = model::DescribeDatasetImportJobResponse {
                    dataset_import_job: Some(dataset_import_job_to_model(j)),
                };
                wire::serialize_describe_dataset_import_job_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_dataset_import_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_dataset_import_jobs_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let dataset_arn = input.dataset_arn.as_deref();
        let state = state.read().await;
        let jobs = state.list_dataset_import_jobs(dataset_arn);
        let entries: Vec<model::DatasetImportJobSummary> = jobs
            .iter()
            .map(|j| dataset_import_job_to_summary(j))
            .collect();
        let resp = model::ListDatasetImportJobsResponse {
            dataset_import_jobs: Some(entries),
            next_token: None,
        };
        wire::serialize_list_dataset_import_jobs_response(&resp)
    }

    // ======================== Metric Attributions ========================

    async fn handle_create_metric_attribution(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_metric_attribution_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "name".to_string(),
            });
        }
        if input.dataset_group_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetGroupArn".to_string(),
            });
        }
        let name = input.name.as_str();
        let dataset_group_arn = input.dataset_group_arn.as_str();

        let metrics: Vec<crate::types::MetricAttributeEntry> = input
            .metrics
            .into_iter()
            .map(|m| crate::types::MetricAttributeEntry {
                event_type: m.event_type,
                expression: m.expression,
                metric_name: m.metric_name,
            })
            .collect();

        let role_arn = input.metrics_output_config.role_arn.as_str();
        let s3_path = input
            .metrics_output_config
            .s3_data_destination
            .as_ref()
            .map(|s| s.path.as_str());

        let mut state = state.write().await;
        match state.create_metric_attribution(
            name,
            dataset_group_arn,
            metrics,
            role_arn,
            s3_path,
            region,
            account_id,
        ) {
            Ok(ma) => {
                let resp = model::CreateMetricAttributionResponse {
                    metric_attribution_arn: Some(ma.metric_attribution_arn.clone()),
                };
                wire::serialize_create_metric_attribution_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_metric_attribution(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_metric_attribution_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.metric_attribution_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "metricAttributionArn".to_string(),
            });
        }
        let arn = input.metric_attribution_arn.as_str();
        let state = state.read().await;
        match state.describe_metric_attribution(arn) {
            Ok(ma) => {
                let resp = model::DescribeMetricAttributionResponse {
                    metric_attribution: Some(metric_attribution_to_model(ma)),
                };
                wire::serialize_describe_metric_attribution_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_delete_metric_attribution(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_metric_attribution_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.metric_attribution_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "metricAttributionArn".to_string(),
            });
        }
        let arn = input.metric_attribution_arn.as_str();
        let mut state = state.write().await;
        match state.delete_metric_attribution(arn) {
            Ok(()) => wire::serialize_delete_metric_attribution_response(),
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_metric_attributions(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_metric_attributions_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let dataset_group_arn = input.dataset_group_arn.as_deref();
        let state = state.read().await;
        let mas = state.list_metric_attributions(dataset_group_arn);
        let entries: Vec<model::MetricAttributionSummary> = mas
            .iter()
            .map(|ma| metric_attribution_to_summary(ma))
            .collect();
        let resp = model::ListMetricAttributionsResponse {
            metric_attributions: Some(entries),
            next_token: None,
        };
        wire::serialize_list_metric_attributions_response(&resp)
    }

    async fn handle_update_metric_attribution(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_metric_attribution_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let arn = match input.metric_attribution_arn.as_deref() {
            Some(a) if !a.is_empty() => a,
            _ => {
                return personalize_error_response(&PersonalizeError::InvalidInput {
                    field: "metricAttributionArn".to_string(),
                });
            }
        };

        let add_metrics: Option<Vec<crate::types::MetricAttributeEntry>> =
            input.add_metrics.map(|v| {
                v.into_iter()
                    .map(|m| crate::types::MetricAttributeEntry {
                        event_type: m.event_type,
                        expression: m.expression,
                        metric_name: m.metric_name,
                    })
                    .collect()
            });

        let remove_metrics = input.remove_metrics;

        let role_arn = input
            .metrics_output_config
            .as_ref()
            .map(|c| c.role_arn.as_str());
        let s3_path = input
            .metrics_output_config
            .as_ref()
            .and_then(|c| c.s3_data_destination.as_ref())
            .map(|s| s.path.as_str());

        let mut state = state.write().await;
        match state.update_metric_attribution(arn, add_metrics, remove_metrics, role_arn, s3_path) {
            Ok(ma) => {
                let resp = model::UpdateMetricAttributionResponse {
                    metric_attribution_arn: Some(ma.metric_attribution_arn.clone()),
                };
                wire::serialize_update_metric_attribution_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_metric_attribution_metrics(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_metric_attribution_metrics_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let metric_attribution_arn = input.metric_attribution_arn.as_deref();
        let state = state.read().await;
        match state.list_metric_attribution_metrics(metric_attribution_arn) {
            Ok(metrics) => {
                let entries: Vec<model::MetricAttribute> = metrics
                    .iter()
                    .map(|m| model::MetricAttribute {
                        event_type: m.event_type.clone(),
                        expression: m.expression.clone(),
                        metric_name: m.metric_name.clone(),
                    })
                    .collect();
                let resp = model::ListMetricAttributionMetricsResponse {
                    metrics: Some(entries),
                    next_token: None,
                };
                wire::serialize_list_metric_attribution_metrics_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    // ======================== Recommenders ========================

    async fn handle_create_recommender(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_recommender_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "name".to_string(),
            });
        }
        if input.dataset_group_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetGroupArn".to_string(),
            });
        }
        if input.recipe_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "recipeArn".to_string(),
            });
        }
        let name = input.name.as_str();
        let dataset_group_arn = input.dataset_group_arn.as_str();
        let recipe_arn = input.recipe_arn.as_str();

        let config = input
            .recommender_config
            .map(|c| crate::types::RecommenderConfigData {
                enable_metadata_with_recommendations: c.enable_metadata_with_recommendations,
                item_exploration_config: c.item_exploration_config,
                min_recommendation_requests_per_second: c.min_recommendation_requests_per_second,
            });

        let mut state = state.write().await;
        match state.create_recommender(
            name,
            dataset_group_arn,
            recipe_arn,
            config,
            region,
            account_id,
        ) {
            Ok(r) => {
                let resp = model::CreateRecommenderResponse {
                    recommender_arn: Some(r.recommender_arn.clone()),
                };
                wire::serialize_create_recommender_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_recommender(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_recommender_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.recommender_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "recommenderArn".to_string(),
            });
        }
        let arn = input.recommender_arn.as_str();
        let state = state.read().await;
        match state.describe_recommender(arn) {
            Ok(r) => {
                let resp = model::DescribeRecommenderResponse {
                    recommender: Some(recommender_to_model(r)),
                };
                wire::serialize_describe_recommender_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_delete_recommender(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_recommender_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.recommender_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "recommenderArn".to_string(),
            });
        }
        let arn = input.recommender_arn.as_str();
        let mut state = state.write().await;
        match state.delete_recommender(arn) {
            Ok(()) => wire::serialize_delete_recommender_response(),
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_recommenders(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_recommenders_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let dataset_group_arn = input.dataset_group_arn.as_deref();
        let state = state.read().await;
        let recommenders = state.list_recommenders(dataset_group_arn);
        let entries: Vec<model::RecommenderSummary> = recommenders
            .iter()
            .map(|r| recommender_to_summary(r))
            .collect();
        let resp = model::ListRecommendersResponse {
            recommenders: Some(entries),
            next_token: None,
        };
        wire::serialize_list_recommenders_response(&resp)
    }

    async fn handle_update_recommender(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_recommender_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.recommender_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "recommenderArn".to_string(),
            });
        }
        let arn = input.recommender_arn.as_str();
        let config = crate::types::RecommenderConfigData {
            enable_metadata_with_recommendations: input
                .recommender_config
                .enable_metadata_with_recommendations,
            item_exploration_config: input.recommender_config.item_exploration_config,
            min_recommendation_requests_per_second: input
                .recommender_config
                .min_recommendation_requests_per_second,
        };

        let mut state = state.write().await;
        match state.update_recommender(arn, config) {
            Ok(r) => {
                let resp = model::UpdateRecommenderResponse {
                    recommender_arn: Some(r.recommender_arn.clone()),
                };
                wire::serialize_update_recommender_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_start_recommender(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_recommender_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.recommender_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "recommenderArn".to_string(),
            });
        }
        let arn = input.recommender_arn.as_str();
        let mut state = state.write().await;
        match state.start_recommender(arn) {
            Ok(r) => {
                let resp = model::StartRecommenderResponse {
                    recommender_arn: Some(r.recommender_arn.clone()),
                };
                wire::serialize_start_recommender_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_stop_recommender(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_recommender_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.recommender_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "recommenderArn".to_string(),
            });
        }
        let arn = input.recommender_arn.as_str();
        let mut state = state.write().await;
        match state.stop_recommender(arn) {
            Ok(r) => {
                let resp = model::StopRecommenderResponse {
                    recommender_arn: Some(r.recommender_arn.clone()),
                };
                wire::serialize_stop_recommender_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    // ======================== Tags ========================

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.resource_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "resourceArn".to_string(),
            });
        }
        let resource_arn = input.resource_arn.as_str();
        let tags: Vec<crate::types::ResourceTag> = input
            .tags
            .into_iter()
            .map(|t| crate::types::ResourceTag {
                tag_key: t.tag_key,
                tag_value: t.tag_value,
            })
            .collect();

        let mut state = state.write().await;
        state.tag_resource(resource_arn, tags);
        let resp = model::TagResourceResponse {};
        wire::serialize_tag_resource_response(&resp)
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.resource_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "resourceArn".to_string(),
            });
        }
        let resource_arn = input.resource_arn.as_str();
        let tag_keys = input.tag_keys;

        let mut state = state.write().await;
        state.untag_resource(resource_arn, &tag_keys);
        let resp = model::UntagResourceResponse {};
        wire::serialize_untag_resource_response(&resp)
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.resource_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "resourceArn".to_string(),
            });
        }
        let resource_arn = input.resource_arn.as_str();
        let state = state.read().await;
        let tags = state.list_tags_for_resource(resource_arn);
        let entries: Vec<model::Tag> = tags
            .iter()
            .map(|t| model::Tag {
                tag_key: t.tag_key.clone(),
                tag_value: t.tag_value.clone(),
            })
            .collect();
        let resp = model::ListTagsForResourceResponse {
            tags: Some(entries),
        };
        wire::serialize_list_tags_for_resource_response(&resp)
    }

    // ======================== Solutions ========================

    async fn handle_create_solution(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_solution_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.name.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "name".to_string(),
            });
        }
        if input.dataset_group_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "datasetGroupArn".to_string(),
            });
        }
        let name = input.name.as_str();
        let dataset_group_arn = input.dataset_group_arn.as_str();
        let recipe_arn = input.recipe_arn.as_deref();
        let event_type = input.event_type.as_deref();
        let perform_auto_ml = input.perform_auto_m_l;
        let perform_auto_training = input.perform_auto_training;
        let perform_hpo = input.perform_h_p_o;

        let mut state = state.write().await;
        match state.create_solution(
            name,
            dataset_group_arn,
            recipe_arn,
            event_type,
            perform_auto_ml,
            perform_auto_training,
            perform_hpo,
            region,
            account_id,
        ) {
            Ok(s) => {
                let resp = model::CreateSolutionResponse {
                    solution_arn: Some(s.solution_arn.clone()),
                };
                wire::serialize_create_solution_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_solution(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_solution_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.solution_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "solutionArn".to_string(),
            });
        }
        let arn = input.solution_arn.as_str();
        let state = state.read().await;
        match state.describe_solution(arn) {
            Ok(s) => {
                let resp = model::DescribeSolutionResponse {
                    solution: Some(solution_to_model(s)),
                };
                wire::serialize_describe_solution_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_delete_solution(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_solution_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.solution_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "solutionArn".to_string(),
            });
        }
        let arn = input.solution_arn.as_str();
        let mut state = state.write().await;
        match state.delete_solution(arn) {
            Ok(()) => wire::serialize_delete_solution_response(),
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_solutions(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_solutions_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let dataset_group_arn = input.dataset_group_arn.as_deref();
        let state = state.read().await;
        let solutions = state.list_solutions(dataset_group_arn);
        let entries: Vec<model::SolutionSummary> =
            solutions.iter().map(|s| solution_to_summary(s)).collect();
        let resp = model::ListSolutionsResponse {
            solutions: Some(entries),
            next_token: None,
        };
        wire::serialize_list_solutions_response(&resp)
    }

    async fn handle_update_solution(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_solution_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.solution_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "solutionArn".to_string(),
            });
        }
        let arn = input.solution_arn.as_str();
        let perform_auto_training = input.perform_auto_training;
        let mut state = state.write().await;
        match state.update_solution(arn, perform_auto_training) {
            Ok(s) => {
                let resp = model::UpdateSolutionResponse {
                    solution_arn: Some(s.solution_arn.clone()),
                };
                wire::serialize_update_solution_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    // ======================== Solution Versions ========================

    async fn handle_create_solution_version(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_solution_version_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.solution_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "solutionArn".to_string(),
            });
        }
        let solution_arn = input.solution_arn.as_str();
        let name = input.name.as_deref();
        let training_mode = input.training_mode.as_deref();

        let mut state = state.write().await;
        match state.create_solution_version(solution_arn, name, training_mode, region, account_id) {
            Ok(sv) => {
                let resp = model::CreateSolutionVersionResponse {
                    solution_version_arn: Some(sv.solution_version_arn.clone()),
                };
                wire::serialize_create_solution_version_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_describe_solution_version(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_solution_version_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.solution_version_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "solutionVersionArn".to_string(),
            });
        }
        let arn = input.solution_version_arn.as_str();
        let state = state.read().await;
        match state.describe_solution_version(arn) {
            Ok(sv) => {
                let resp = model::DescribeSolutionVersionResponse {
                    solution_version: Some(solution_version_to_model(sv)),
                };
                wire::serialize_describe_solution_version_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    async fn handle_list_solution_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_solution_versions_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        let solution_arn = input.solution_arn.as_deref();
        let state = state.read().await;
        let versions = state.list_solution_versions(solution_arn);
        let entries: Vec<model::SolutionVersionSummary> = versions
            .iter()
            .map(|sv| solution_version_to_summary(sv))
            .collect();
        let resp = model::ListSolutionVersionsResponse {
            solution_versions: Some(entries),
            next_token: None,
        };
        wire::serialize_list_solution_versions_response(&resp)
    }

    async fn handle_stop_solution_version_creation(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_solution_version_creation_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.solution_version_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "solutionVersionArn".to_string(),
            });
        }
        let arn = input.solution_version_arn.as_str();
        let mut state = state.write().await;
        match state.stop_solution_version_creation(arn) {
            Ok(()) => wire::serialize_stop_solution_version_creation_response(),
            Err(e) => personalize_error_response(&e),
        }
    }

    // STUB[no-engine]: GetSolutionMetrics requires a real ML training engine to produce
    //   meaningful accuracy metrics; always returns an empty metrics map.
    async fn handle_get_solution_metrics(
        &self,
        state: &Arc<tokio::sync::RwLock<PersonalizeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_solution_metrics_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.solution_version_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "solutionVersionArn".to_string(),
            });
        }
        let arn = input.solution_version_arn.as_str();
        let state = state.read().await;
        match state.describe_solution_version(arn) {
            Ok(_sv) => {
                // Return empty metrics since we don't have a real training engine
                let resp = model::GetSolutionMetricsResponse {
                    solution_version_arn: Some(arn.to_string()),
                    metrics: Some(std::collections::HashMap::new()),
                };
                wire::serialize_get_solution_metrics_response(&resp)
            }
            Err(e) => personalize_error_response(&e),
        }
    }

    // ======================== Built-in Catalogue Stubs ========================

    // STUB[no-engine]: DescribeAlgorithm reads from the AWS-managed algorithm catalogue;
    //   the mock has no catalogue data, so only the requested ARN is echoed back.
    async fn handle_describe_algorithm(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_algorithm_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.algorithm_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "algorithmArn".to_string(),
            });
        }
        let arn = input.algorithm_arn.as_str();
        let resp = model::DescribeAlgorithmResponse {
            algorithm: Some(model::Algorithm {
                algorithm_arn: Some(arn.to_string()),
                name: Some(arn.rsplit('/').next().unwrap_or(arn).to_string()),
                ..Default::default()
            }),
        };
        wire::serialize_describe_algorithm_response(&resp)
    }

    // STUB[no-engine]: DescribeFeatureTransformation reads from the AWS-managed catalogue;
    //   the mock has no catalogue data, so only the requested ARN is echoed back.
    async fn handle_describe_feature_transformation(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_feature_transformation_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.feature_transformation_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "featureTransformationArn".to_string(),
            });
        }
        let arn = input.feature_transformation_arn.as_str();
        let resp = model::DescribeFeatureTransformationResponse {
            feature_transformation: Some(model::FeatureTransformation {
                feature_transformation_arn: Some(arn.to_string()),
                name: Some(arn.rsplit('/').next().unwrap_or(arn).to_string()),
                ..Default::default()
            }),
        };
        wire::serialize_describe_feature_transformation_response(&resp)
    }

    // STUB[no-engine]: DescribeRecipe reads from the AWS-managed recipe catalogue;
    //   the mock has no catalogue data, so only the requested ARN is echoed back.
    async fn handle_describe_recipe(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_recipe_request(body) {
            Ok(v) => v,
            Err(_) => return personalize_error_response(&PersonalizeError::SerializationException),
        };
        if input.recipe_arn.is_empty() {
            return personalize_error_response(&PersonalizeError::InvalidInput {
                field: "recipeArn".to_string(),
            });
        }
        let arn = input.recipe_arn.as_str();
        let resp = model::DescribeRecipeResponse {
            recipe: Some(model::Recipe {
                recipe_arn: Some(arn.to_string()),
                name: Some(arn.rsplit('/').next().unwrap_or(arn).to_string()),
                ..Default::default()
            }),
        };
        wire::serialize_describe_recipe_response(&resp)
    }

    // STUB[no-engine]: ListRecipes reads from the AWS-managed recipe catalogue;
    //   the mock has no catalogue data, so always returns an empty list.
    async fn handle_list_recipes(&self) -> MockResponse {
        let resp = model::ListRecipesResponse {
            recipes: Some(vec![]),
            next_token: None,
        };
        wire::serialize_list_recipes_response(&resp)
    }
}

// ======================== Model conversion helpers ========================

fn dataset_group_to_model(dg: &crate::types::DatasetGroup) -> model::DatasetGroup {
    model::DatasetGroup {
        name: Some(dg.name.clone()),
        dataset_group_arn: Some(dg.dataset_group_arn.clone()),
        status: Some(dg.status.clone()),
        role_arn: dg.role_arn.clone(),
        kms_key_arn: dg.kms_key_arn.clone(),
        domain: dg.domain.clone(),
        creation_date_time: Some(dg.creation_date_time),
        last_updated_date_time: Some(dg.last_updated_date_time),
        ..Default::default()
    }
}

fn dataset_group_to_summary(dg: &crate::types::DatasetGroup) -> model::DatasetGroupSummary {
    model::DatasetGroupSummary {
        name: Some(dg.name.clone()),
        dataset_group_arn: Some(dg.dataset_group_arn.clone()),
        status: Some(dg.status.clone()),
        domain: dg.domain.clone(),
        creation_date_time: Some(dg.creation_date_time),
        last_updated_date_time: Some(dg.last_updated_date_time),
        ..Default::default()
    }
}

fn schema_to_model(s: &crate::types::Schema) -> model::DatasetSchema {
    model::DatasetSchema {
        name: Some(s.name.clone()),
        schema_arn: Some(s.schema_arn.clone()),
        schema: Some(s.schema.clone()),
        domain: s.domain.clone(),
        creation_date_time: Some(s.creation_date_time),
        last_updated_date_time: Some(s.last_updated_date_time),
    }
}

fn schema_to_summary(s: &crate::types::Schema) -> model::DatasetSchemaSummary {
    model::DatasetSchemaSummary {
        name: Some(s.name.clone()),
        schema_arn: Some(s.schema_arn.clone()),
        domain: s.domain.clone(),
        creation_date_time: Some(s.creation_date_time),
        last_updated_date_time: Some(s.last_updated_date_time),
    }
}

fn dataset_to_model(ds: &crate::types::Dataset) -> model::Dataset {
    model::Dataset {
        name: Some(ds.name.clone()),
        dataset_arn: Some(ds.dataset_arn.clone()),
        dataset_group_arn: Some(ds.dataset_group_arn.clone()),
        dataset_type: Some(ds.dataset_type.clone()),
        schema_arn: Some(ds.schema_arn.clone()),
        status: Some(ds.status.clone()),
        creation_date_time: Some(ds.creation_date_time),
        last_updated_date_time: Some(ds.last_updated_date_time),
        ..Default::default()
    }
}

fn dataset_to_summary(ds: &crate::types::Dataset) -> model::DatasetSummary {
    model::DatasetSummary {
        name: Some(ds.name.clone()),
        dataset_arn: Some(ds.dataset_arn.clone()),
        dataset_type: Some(ds.dataset_type.clone()),
        status: Some(ds.status.clone()),
        creation_date_time: Some(ds.creation_date_time),
        last_updated_date_time: Some(ds.last_updated_date_time),
    }
}

fn campaign_to_model(c: &crate::types::Campaign) -> model::Campaign {
    model::Campaign {
        name: Some(c.name.clone()),
        campaign_arn: Some(c.campaign_arn.clone()),
        solution_version_arn: Some(c.solution_version_arn.clone()),
        min_provisioned_t_p_s: c.min_provisioned_tps,
        status: Some(c.status.clone()),
        creation_date_time: Some(c.creation_date_time),
        last_updated_date_time: Some(c.last_updated_date_time),
        ..Default::default()
    }
}

fn campaign_to_summary(c: &crate::types::Campaign) -> model::CampaignSummary {
    model::CampaignSummary {
        name: Some(c.name.clone()),
        campaign_arn: Some(c.campaign_arn.clone()),
        status: Some(c.status.clone()),
        creation_date_time: Some(c.creation_date_time),
        last_updated_date_time: Some(c.last_updated_date_time),
        ..Default::default()
    }
}

fn event_tracker_to_model(et: &crate::types::EventTracker) -> model::EventTracker {
    model::EventTracker {
        name: Some(et.name.clone()),
        event_tracker_arn: Some(et.event_tracker_arn.clone()),
        dataset_group_arn: Some(et.dataset_group_arn.clone()),
        tracking_id: Some(et.tracking_id.clone()),
        account_id: Some(et.account_id.clone()),
        status: Some(et.status.clone()),
        creation_date_time: Some(et.creation_date_time),
        last_updated_date_time: Some(et.last_updated_date_time),
    }
}

fn event_tracker_to_summary(et: &crate::types::EventTracker) -> model::EventTrackerSummary {
    model::EventTrackerSummary {
        name: Some(et.name.clone()),
        event_tracker_arn: Some(et.event_tracker_arn.clone()),
        status: Some(et.status.clone()),
        creation_date_time: Some(et.creation_date_time),
        last_updated_date_time: Some(et.last_updated_date_time),
    }
}

fn filter_to_model(f: &crate::types::Filter) -> model::Filter {
    model::Filter {
        name: Some(f.name.clone()),
        filter_arn: Some(f.filter_arn.clone()),
        dataset_group_arn: Some(f.dataset_group_arn.clone()),
        filter_expression: Some(f.filter_expression.clone()),
        status: Some(f.status.clone()),
        creation_date_time: Some(f.creation_date_time),
        last_updated_date_time: Some(f.last_updated_date_time),
        ..Default::default()
    }
}

fn filter_to_summary(f: &crate::types::Filter) -> model::FilterSummary {
    model::FilterSummary {
        name: Some(f.name.clone()),
        filter_arn: Some(f.filter_arn.clone()),
        dataset_group_arn: Some(f.dataset_group_arn.clone()),
        status: Some(f.status.clone()),
        creation_date_time: Some(f.creation_date_time),
        last_updated_date_time: Some(f.last_updated_date_time),
        ..Default::default()
    }
}

fn batch_inference_job_to_model(j: &crate::types::BatchInferenceJob) -> model::BatchInferenceJob {
    model::BatchInferenceJob {
        job_name: Some(j.job_name.clone()),
        batch_inference_job_arn: Some(j.batch_inference_job_arn.clone()),
        solution_version_arn: Some(j.solution_version_arn.clone()),
        filter_arn: j.filter_arn.clone(),
        role_arn: Some(j.role_arn.clone()),
        status: Some(j.status.clone()),
        num_results: j.num_results,
        job_input: Some(model::BatchInferenceJobInput {
            s3_data_source: model::S3DataConfig {
                path: j.job_input_s3_path.clone(),
                ..Default::default()
            },
        }),
        job_output: Some(model::BatchInferenceJobOutput {
            s3_data_destination: model::S3DataConfig {
                path: j.job_output_s3_path.clone(),
                ..Default::default()
            },
        }),
        batch_inference_job_mode: j.batch_inference_job_mode.clone(),
        creation_date_time: Some(j.creation_date_time),
        last_updated_date_time: Some(j.last_updated_date_time),
        ..Default::default()
    }
}

fn batch_inference_job_to_summary(
    j: &crate::types::BatchInferenceJob,
) -> model::BatchInferenceJobSummary {
    model::BatchInferenceJobSummary {
        job_name: Some(j.job_name.clone()),
        batch_inference_job_arn: Some(j.batch_inference_job_arn.clone()),
        solution_version_arn: Some(j.solution_version_arn.clone()),
        batch_inference_job_mode: j.batch_inference_job_mode.clone(),
        status: Some(j.status.clone()),
        creation_date_time: Some(j.creation_date_time),
        last_updated_date_time: Some(j.last_updated_date_time),
        ..Default::default()
    }
}

fn batch_segment_job_to_model(j: &crate::types::BatchSegmentJob) -> model::BatchSegmentJob {
    model::BatchSegmentJob {
        job_name: Some(j.job_name.clone()),
        batch_segment_job_arn: Some(j.batch_segment_job_arn.clone()),
        solution_version_arn: Some(j.solution_version_arn.clone()),
        filter_arn: j.filter_arn.clone(),
        role_arn: Some(j.role_arn.clone()),
        num_results: j.num_results,
        job_input: Some(model::BatchSegmentJobInput {
            s3_data_source: model::S3DataConfig {
                path: j.job_input_s3_path.clone(),
                ..Default::default()
            },
        }),
        job_output: Some(model::BatchSegmentJobOutput {
            s3_data_destination: model::S3DataConfig {
                path: j.job_output_s3_path.clone(),
                ..Default::default()
            },
        }),
        status: Some(j.status.clone()),
        creation_date_time: Some(j.creation_date_time),
        last_updated_date_time: Some(j.last_updated_date_time),
        ..Default::default()
    }
}

fn batch_segment_job_to_summary(
    j: &crate::types::BatchSegmentJob,
) -> model::BatchSegmentJobSummary {
    model::BatchSegmentJobSummary {
        job_name: Some(j.job_name.clone()),
        batch_segment_job_arn: Some(j.batch_segment_job_arn.clone()),
        solution_version_arn: Some(j.solution_version_arn.clone()),
        status: Some(j.status.clone()),
        creation_date_time: Some(j.creation_date_time),
        last_updated_date_time: Some(j.last_updated_date_time),
        ..Default::default()
    }
}

fn data_deletion_job_to_model(j: &crate::types::DataDeletionJob) -> model::DataDeletionJob {
    model::DataDeletionJob {
        job_name: Some(j.job_name.clone()),
        data_deletion_job_arn: Some(j.data_deletion_job_arn.clone()),
        dataset_group_arn: Some(j.dataset_group_arn.clone()),
        data_source: Some(model::DataSource {
            data_location: Some(j.data_source_location.clone()),
        }),
        role_arn: Some(j.role_arn.clone()),
        status: Some(j.status.clone()),
        creation_date_time: Some(j.creation_date_time),
        last_updated_date_time: Some(j.last_updated_date_time),
        ..Default::default()
    }
}

fn data_deletion_job_to_summary(
    j: &crate::types::DataDeletionJob,
) -> model::DataDeletionJobSummary {
    model::DataDeletionJobSummary {
        job_name: Some(j.job_name.clone()),
        data_deletion_job_arn: Some(j.data_deletion_job_arn.clone()),
        dataset_group_arn: Some(j.dataset_group_arn.clone()),
        status: Some(j.status.clone()),
        creation_date_time: Some(j.creation_date_time),
        last_updated_date_time: Some(j.last_updated_date_time),
        ..Default::default()
    }
}

fn dataset_export_job_to_model(j: &crate::types::DatasetExportJob) -> model::DatasetExportJob {
    model::DatasetExportJob {
        job_name: Some(j.job_name.clone()),
        dataset_export_job_arn: Some(j.dataset_export_job_arn.clone()),
        dataset_arn: Some(j.dataset_arn.clone()),
        role_arn: Some(j.role_arn.clone()),
        ingestion_mode: j.ingestion_mode.clone(),
        job_output: Some(model::DatasetExportJobOutput {
            s3_data_destination: model::S3DataConfig {
                path: j.job_output_s3_path.clone(),
                ..Default::default()
            },
        }),
        status: Some(j.status.clone()),
        creation_date_time: Some(j.creation_date_time),
        last_updated_date_time: Some(j.last_updated_date_time),
        ..Default::default()
    }
}

fn dataset_export_job_to_summary(
    j: &crate::types::DatasetExportJob,
) -> model::DatasetExportJobSummary {
    model::DatasetExportJobSummary {
        job_name: Some(j.job_name.clone()),
        dataset_export_job_arn: Some(j.dataset_export_job_arn.clone()),
        status: Some(j.status.clone()),
        creation_date_time: Some(j.creation_date_time),
        last_updated_date_time: Some(j.last_updated_date_time),
        ..Default::default()
    }
}

fn dataset_import_job_to_model(j: &crate::types::DatasetImportJob) -> model::DatasetImportJob {
    model::DatasetImportJob {
        job_name: Some(j.job_name.clone()),
        dataset_import_job_arn: Some(j.dataset_import_job_arn.clone()),
        dataset_arn: Some(j.dataset_arn.clone()),
        data_source: Some(model::DataSource {
            data_location: Some(j.data_source_location.clone()),
        }),
        role_arn: j.role_arn.clone(),
        import_mode: j.import_mode.clone(),
        status: Some(j.status.clone()),
        creation_date_time: Some(j.creation_date_time),
        last_updated_date_time: Some(j.last_updated_date_time),
        ..Default::default()
    }
}

fn dataset_import_job_to_summary(
    j: &crate::types::DatasetImportJob,
) -> model::DatasetImportJobSummary {
    model::DatasetImportJobSummary {
        job_name: Some(j.job_name.clone()),
        dataset_import_job_arn: Some(j.dataset_import_job_arn.clone()),
        import_mode: j.import_mode.clone(),
        status: Some(j.status.clone()),
        creation_date_time: Some(j.creation_date_time),
        last_updated_date_time: Some(j.last_updated_date_time),
        ..Default::default()
    }
}

fn metric_attribution_to_model(ma: &crate::types::MetricAttribution) -> model::MetricAttribution {
    model::MetricAttribution {
        name: Some(ma.name.clone()),
        metric_attribution_arn: Some(ma.metric_attribution_arn.clone()),
        dataset_group_arn: Some(ma.dataset_group_arn.clone()),
        metrics_output_config: Some(model::MetricAttributionOutput {
            role_arn: ma.metrics_output_role_arn.clone(),
            s3_data_destination: ma
                .metrics_output_s3_path
                .as_ref()
                .map(|p| model::S3DataConfig {
                    path: p.clone(),
                    ..Default::default()
                }),
        }),
        status: Some(ma.status.clone()),
        creation_date_time: Some(ma.creation_date_time),
        last_updated_date_time: Some(ma.last_updated_date_time),
        ..Default::default()
    }
}

fn metric_attribution_to_summary(
    ma: &crate::types::MetricAttribution,
) -> model::MetricAttributionSummary {
    model::MetricAttributionSummary {
        name: Some(ma.name.clone()),
        metric_attribution_arn: Some(ma.metric_attribution_arn.clone()),
        status: Some(ma.status.clone()),
        creation_date_time: Some(ma.creation_date_time),
        last_updated_date_time: Some(ma.last_updated_date_time),
        ..Default::default()
    }
}

fn recommender_config_to_model(
    c: &crate::types::RecommenderConfigData,
) -> model::RecommenderConfig {
    model::RecommenderConfig {
        enable_metadata_with_recommendations: c.enable_metadata_with_recommendations,
        item_exploration_config: c.item_exploration_config.clone(),
        min_recommendation_requests_per_second: c.min_recommendation_requests_per_second,
        ..Default::default()
    }
}

fn recommender_to_model(r: &crate::types::Recommender) -> model::Recommender {
    model::Recommender {
        name: Some(r.name.clone()),
        recommender_arn: Some(r.recommender_arn.clone()),
        dataset_group_arn: Some(r.dataset_group_arn.clone()),
        recipe_arn: Some(r.recipe_arn.clone()),
        recommender_config: r
            .recommender_config
            .as_ref()
            .map(recommender_config_to_model),
        status: Some(r.status.clone()),
        creation_date_time: Some(r.creation_date_time),
        last_updated_date_time: Some(r.last_updated_date_time),
        ..Default::default()
    }
}

fn recommender_to_summary(r: &crate::types::Recommender) -> model::RecommenderSummary {
    model::RecommenderSummary {
        name: Some(r.name.clone()),
        recommender_arn: Some(r.recommender_arn.clone()),
        dataset_group_arn: Some(r.dataset_group_arn.clone()),
        recipe_arn: Some(r.recipe_arn.clone()),
        recommender_config: r
            .recommender_config
            .as_ref()
            .map(recommender_config_to_model),
        status: Some(r.status.clone()),
        creation_date_time: Some(r.creation_date_time),
        last_updated_date_time: Some(r.last_updated_date_time),
    }
}

fn solution_to_model(s: &crate::types::Solution) -> model::Solution {
    model::Solution {
        name: Some(s.name.clone()),
        solution_arn: Some(s.solution_arn.clone()),
        dataset_group_arn: Some(s.dataset_group_arn.clone()),
        recipe_arn: s.recipe_arn.clone(),
        event_type: s.event_type.clone(),
        perform_auto_m_l: s.perform_auto_ml,
        perform_auto_training: s.perform_auto_training,
        perform_h_p_o: s.perform_hpo,
        status: Some(s.status.clone()),
        creation_date_time: Some(s.creation_date_time),
        last_updated_date_time: Some(s.last_updated_date_time),
        ..Default::default()
    }
}

fn solution_to_summary(s: &crate::types::Solution) -> model::SolutionSummary {
    model::SolutionSummary {
        name: Some(s.name.clone()),
        solution_arn: Some(s.solution_arn.clone()),
        recipe_arn: s.recipe_arn.clone(),
        status: Some(s.status.clone()),
        creation_date_time: Some(s.creation_date_time),
        last_updated_date_time: Some(s.last_updated_date_time),
    }
}

fn solution_version_to_model(sv: &crate::types::SolutionVersionData) -> model::SolutionVersion {
    model::SolutionVersion {
        solution_version_arn: Some(sv.solution_version_arn.clone()),
        solution_arn: Some(sv.solution_arn.clone()),
        name: sv.name.clone(),
        training_mode: sv.training_mode.clone(),
        status: Some(sv.status.clone()),
        creation_date_time: Some(sv.creation_date_time),
        last_updated_date_time: Some(sv.last_updated_date_time),
        ..Default::default()
    }
}

fn solution_version_to_summary(
    sv: &crate::types::SolutionVersionData,
) -> model::SolutionVersionSummary {
    model::SolutionVersionSummary {
        solution_version_arn: Some(sv.solution_version_arn.clone()),
        training_mode: sv.training_mode.clone(),
        status: Some(sv.status.clone()),
        creation_date_time: Some(sv.creation_date_time),
        last_updated_date_time: Some(sv.last_updated_date_time),
        ..Default::default()
    }
}

fn personalize_error_response(err: &PersonalizeError) -> MockResponse {
    let (status, error_type) = match err {
        PersonalizeError::ResourceAlreadyExists { .. } => (400, "ResourceAlreadyExistsException"),
        PersonalizeError::ResourceNotFound { .. } => (400, "ResourceNotFoundException"),
        PersonalizeError::MissingAction => (400, "MissingActionException"),
        PersonalizeError::SerializationException => (400, "SerializationException"),
        PersonalizeError::InvalidInput { .. } => (400, "InvalidInputException"),
        PersonalizeError::InvalidAction { .. } => (400, "UnknownOperationException"),
        PersonalizeError::ResourceInUse { .. } => (400, "ResourceInUseException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}
