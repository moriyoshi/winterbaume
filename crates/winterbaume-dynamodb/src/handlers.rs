use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::backend::{DynamoDbBackend, InMemoryDynamoDbBackend};
use crate::state::DynamoDbError;
use crate::types::*;
use crate::views::DynamodbStateView;
use crate::wire;

/// AWS-fidelity validation for the WHERE clause of an `EXISTS(SELECT …)`
/// inner statement. Real DynamoDB requires:
/// 1. The hash key referenced by an equality predicate.
/// 2. The range key referenced by an equality predicate (when one exists).
/// 3. At least one non-key predicate ("additional condition").
///
/// Returns `Ok(())` on success or an error string matching the wording
/// AWS returns. Verified empirically against real DDB on 2026-04-29.
fn validate_exists_where(
    conditions: &[winterbaume_partiql::Condition],
    hash_key: &str,
    range_key: Option<&str>,
) -> Result<(), String> {
    use winterbaume_partiql::Condition;
    use winterbaume_partiql::operation::CmpOp;

    let mut has_hash = false;
    // No range key on the table → satisfied vacuously.
    let mut has_range = range_key.is_none();
    let mut has_non_key = false;

    for cond in conditions {
        match cond {
            Condition::Compare(lhs, CmpOp::Eq, _) => {
                let attr = lhs.as_path();
                if attr == Some(hash_key) {
                    has_hash = true;
                } else if range_key.is_some() && attr == range_key {
                    has_range = true;
                } else {
                    has_non_key = true;
                }
            }
            // Any other condition shape (Between, In, IsMissing, etc.)
            // counts as a non-key predicate. Hash/range key constraints
            // require Eq specifically.
            _ => {
                has_non_key = true;
            }
        }
    }

    if !has_hash || !has_range || !has_non_key {
        return Err(
            "EXISTS() must contain a single item read with additional condition".to_string(),
        );
    }
    Ok(())
}

/// AWS-fidelity validation for forbidden expression shapes inside a
/// PartiQL WHERE clause. Real DynamoDB rejects `+`/`-` arithmetic in
/// condition-expression operands with the wording
/// `"Unsupported operator in Condition Expression. Operator: +"` (or `-`),
/// and rejects unary minus over a path with
/// `"Incorrect number of operands for operator or function; operator or function: -, number of operands: 1"`.
///
/// The hand-rolled parser produces a maximally general IR so a single AST
/// shape can cover both the WHERE-clause case (forbidden) and the SET
/// right-hand side (allowed); rejection happens here at runtime rather
/// than at parse time. Walks every WHERE/key-condition vector in the
/// supplied [`DdbOperation`] (including the inner SELECT of an EXISTS()).
fn validate_partiql_op(op: &winterbaume_partiql::DdbOperation) -> Result<(), String> {
    use winterbaume_partiql::DdbOperation;
    match op {
        DdbOperation::Select(s) => {
            if let Some(conds) = &s.where_clause {
                winterbaume_partiql::validate_where(conds)?;
            }
        }
        DdbOperation::Update(u) => {
            winterbaume_partiql::validate_where(&u.key_conditions)?;
        }
        DdbOperation::Delete(d) => {
            winterbaume_partiql::validate_where(&d.key_conditions)?;
        }
        DdbOperation::Exists(inner) => {
            if let Some(conds) = &inner.where_clause {
                winterbaume_partiql::validate_where(conds)?;
            }
        }
        DdbOperation::Insert(_) => {}
    }
    Ok(())
}

/// DynamoDB service handler that processes awsJson1.0 protocol requests.
pub struct DynamoDbService {
    /// Pluggable storage backend for all table, item, and state operations.
    pub(crate) backend: Arc<dyn DynamoDbBackend>,
    pub(crate) notifier: StateChangeNotifier<DynamodbStateView>,
}

impl DynamoDbService {
    pub fn new() -> Self {
        Self {
            backend: Arc::new(InMemoryDynamoDbBackend::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Return all `(account_id, region)` pairs that currently have state.
    pub fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.backend.scopes_with_state()
    }

    /// Create a DynamoDB service backed by a custom storage backend.
    pub fn with_backend(backend: Arc<dyn DynamoDbBackend>) -> Self {
        Self {
            backend,
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for DynamoDbService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DynamoDbService {
    fn service_name(&self) -> &str {
        "dynamodb"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://dynamodb\..*\.amazonaws\.com",
            r"https?://dynamodb\.amazonaws\.com",
            r"https?://.*\.ddb\..*\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl DynamoDbService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = extract_dynamodb_region(&request);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "DynamoDB_20120810.CreateTable"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#MissingAction",
                    "Missing X-Amz-Target header",
                );
            }
        };

        // Validate that the request body is well-formed JSON before
        // dispatching, so all handlers can rely on the codegen
        // `deserialize_*` helpers without re-checking syntactic JSON
        // errors.
        if !request.body.is_empty()
            && let Err(_) = serde_json::from_slice::<Value>(&request.body)
        {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#SerializationException",
                "Invalid JSON body",
            );
        }
        let body_bytes: &[u8] = &request.body;

        let acct = account_id.to_string();
        let rgn = region.clone();

        let response = match action.as_str() {
            "CreateTable" => self.handle_create_table(body_bytes, acct, rgn).await,
            "DeleteTable" => self.handle_delete_table(body_bytes, acct, rgn).await,
            "DescribeTable" => self.handle_describe_table(body_bytes, acct, rgn).await,
            "PutItem" => self.handle_put_item(body_bytes, acct, rgn).await,
            "GetItem" => self.handle_get_item(body_bytes, acct, rgn).await,
            "DeleteItem" => self.handle_delete_item(body_bytes, acct, rgn).await,
            "Query" => self.handle_query(body_bytes, acct, rgn).await,
            "Scan" => self.handle_scan(body_bytes, acct, rgn).await,
            "ListTables" => self.handle_list_tables(body_bytes, acct, rgn).await,
            "UpdateItem" => self.handle_update_item(body_bytes, acct, rgn).await,
            "BatchWriteItem" => self.handle_batch_write_item(body_bytes, acct, rgn).await,
            "BatchGetItem" => self.handle_batch_get_item(body_bytes, acct, rgn).await,
            "UpdateTable" => self.handle_update_table(body_bytes, acct, rgn).await,
            "CreateBackup" => self.handle_create_backup(body_bytes, acct, rgn).await,
            "DeleteBackup" => self.handle_delete_backup(body_bytes, acct, rgn).await,
            "DescribeBackup" => self.handle_describe_backup(body_bytes, acct, rgn).await,
            "ListBackups" => self.handle_list_backups(body_bytes, acct, rgn).await,
            "RestoreTableFromBackup" => {
                self.handle_restore_table_from_backup(body_bytes, acct, rgn)
                    .await
            }
            "RestoreTableToPointInTime" => {
                self.handle_restore_table_to_point_in_time(body_bytes, acct, rgn)
                    .await
            }
            "TagResource" => self.handle_tag_resource(body_bytes, acct, rgn).await,
            "UntagResource" => self.handle_untag_resource(body_bytes, acct, rgn).await,
            "ListTagsOfResource" => {
                self.handle_list_tags_of_resource(body_bytes, acct, rgn)
                    .await
            }
            "UpdateTimeToLive" => self.handle_update_time_to_live(body_bytes, acct, rgn).await,
            "DescribeTimeToLive" => {
                self.handle_describe_time_to_live(body_bytes, acct, rgn)
                    .await
            }
            "UpdateContinuousBackups" => {
                self.handle_update_continuous_backups(body_bytes, acct, rgn)
                    .await
            }
            "DescribeContinuousBackups" => {
                self.handle_describe_continuous_backups(body_bytes, acct, rgn)
                    .await
            }
            "PutResourcePolicy" => self.handle_put_resource_policy(body_bytes, acct, rgn).await,
            "GetResourcePolicy" => self.handle_get_resource_policy(body_bytes, acct, rgn).await,
            "DeleteResourcePolicy" => {
                self.handle_delete_resource_policy(body_bytes, acct, rgn)
                    .await
            }
            "DescribeEndpoints" => self.handle_describe_endpoints(&region).await,
            "TransactGetItems" => self.handle_transact_get_items(body_bytes, acct, rgn).await,
            "TransactWriteItems" => {
                self.handle_transact_write_items(body_bytes, acct, rgn)
                    .await
            }
            "BatchExecuteStatement" => {
                self.handle_batch_execute_statement(body_bytes, acct, rgn)
                    .await
            }
            "ExecuteStatement" => self.handle_execute_statement(body_bytes, acct, rgn).await,
            "ExecuteTransaction" => self.handle_execute_transaction(body_bytes, acct, rgn).await,
            "DescribeExport" => {
                self.handle_describe_export(body_bytes, acct.clone(), rgn.clone())
                    .await
            }
            "DescribeImport" => {
                self.handle_describe_import(body_bytes, acct.clone(), rgn.clone())
                    .await
            }
            "ImportTable" => {
                self.handle_import_table(body_bytes, acct.clone(), rgn.clone())
                    .await
            }
            "ListExports" => self.handle_list_exports(acct.clone(), rgn.clone()).await,
            // Global Tables (v1 API)
            "CreateGlobalTable" => self.handle_create_global_table(body_bytes, acct, rgn).await,
            "DescribeGlobalTable" => {
                self.handle_describe_global_table(body_bytes, acct, rgn)
                    .await
            }
            "DescribeGlobalTableSettings" => {
                self.handle_describe_global_table_settings(body_bytes, acct, rgn)
                    .await
            }
            "UpdateGlobalTable" => self.handle_update_global_table(body_bytes, acct, rgn).await,
            "UpdateGlobalTableSettings" => {
                self.handle_update_global_table_settings(body_bytes, acct, rgn)
                    .await
            }
            "ListGlobalTables" => self.handle_list_global_tables(acct, rgn).await,
            // Kinesis Streaming
            "EnableKinesisStreamingDestination" => {
                self.handle_enable_kinesis_streaming_destination(body_bytes, acct, rgn)
                    .await
            }
            "DisableKinesisStreamingDestination" => {
                self.handle_disable_kinesis_streaming_destination(body_bytes, acct, rgn)
                    .await
            }
            "DescribeKinesisStreamingDestination" => {
                self.handle_describe_kinesis_streaming_destination(body_bytes, acct, rgn)
                    .await
            }
            "UpdateKinesisStreamingDestination" => {
                self.handle_update_kinesis_streaming_destination(body_bytes, acct, rgn)
                    .await
            }
            // Contributor Insights
            "DescribeContributorInsights" => {
                self.handle_describe_contributor_insights(body_bytes, acct, rgn)
                    .await
            }
            "UpdateContributorInsights" => {
                self.handle_update_contributor_insights(body_bytes, acct, rgn)
                    .await
            }
            "ListContributorInsights" => {
                self.handle_list_contributor_insights(body_bytes, acct, rgn)
                    .await
            }
            // Remaining
            "DescribeLimits" => self.handle_describe_limits().await,
            "ListImports" => self.handle_list_imports(acct.clone(), rgn.clone()).await,
            "ExportTableToPointInTime" => {
                self.handle_export_table_to_point_in_time(body_bytes, acct, rgn)
                    .await
            }
            "DescribeTableReplicaAutoScaling" => {
                self.handle_describe_table_replica_auto_scaling(body_bytes, acct, rgn)
                    .await
            }
            "UpdateTableReplicaAutoScaling" => {
                self.handle_update_table_replica_auto_scaling(body_bytes, acct, rgn)
                    .await
            }
            _ => json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#UnknownOperationException",
                &format!("Unknown operation: {action}"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_table(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_create_table_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };

        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }

        let key_schema = key_schema_from_wire(input.key_schema.as_deref().unwrap_or(&[]));
        let attr_defs =
            attribute_definitions_from_wire(input.attribute_definitions.as_deref().unwrap_or(&[]));
        let billing_mode = input.billing_mode.clone();

        let provisioned = input
            .provisioned_throughput
            .as_ref()
            .map(|pt| ProvisionedThroughput {
                read_capacity_units: pt.read_capacity_units,
                write_capacity_units: pt.write_capacity_units,
            });

        let (stream_enabled, stream_view_type) = match &input.stream_specification {
            Some(spec) => (spec.stream_enabled, spec.stream_view_type.clone()),
            None => (false, None),
        };

        let gsi_defs = gsi_defs_from_wire(input.global_secondary_indexes.as_deref().unwrap_or(&[]));
        let lsi_defs = lsi_defs_from_wire(input.local_secondary_indexes.as_deref().unwrap_or(&[]));

        match self
            .backend
            .create_table(
                account_id,
                region,
                input.table_name,
                key_schema,
                attr_defs,
                billing_mode,
                provisioned,
                stream_enabled,
                stream_view_type,
                gsi_defs,
                lsi_defs,
            )
            .await
        {
            Ok(table) => {
                let output = wire::CreateTableOutput {
                    table_description: Some(table_to_wire_description(&table)),
                };
                wire::serialize_create_table_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_delete_table(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_table_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }

        match self
            .backend
            .delete_table(account_id, region, input.table_name)
            .await
        {
            Ok(table) => {
                let desc = table_to_wire_description(&table);
                let output = wire::DeleteTableOutput {
                    table_description: Some(desc),
                };
                wire::serialize_delete_table_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_describe_table(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_table_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }

        match self
            .backend
            .describe_table(account_id, region, input.table_name)
            .await
        {
            Ok(table) => {
                let output = wire::DescribeTableOutput {
                    table: Some(table_to_wire_description(&table)),
                };
                wire::serialize_describe_table_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_put_item(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_put_item_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        if input.item.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing Item",
            );
        }
        let table_name = input.table_name;
        let item: Item = item_from_wire(input.item);
        let return_values = input.return_values.as_deref().unwrap_or("NONE");

        // --- ConditionExpression ---
        let expr_names = input.expression_attribute_names.unwrap_or_default();
        let expr_values = attr_map_from_wire(input.expression_attribute_values);
        let cond = match input.condition_expression.as_deref() {
            Some(s) => match crate::expr::parse_expression(s, &expr_names, &expr_values) {
                Ok(e) => Some(e),
                Err(msg) => {
                    return json_error_response(
                        400,
                        "com.amazonaws.dynamodb.v20120810#ValidationException",
                        &msg,
                    );
                }
            },
            None => None,
        };
        if let Some(ref cond_expr) = cond {
            let existing = self
                .backend
                .get_item(
                    account_id.clone(),
                    region.clone(),
                    table_name.clone(),
                    item.clone(),
                )
                .await
                .ok()
                .flatten()
                .unwrap_or_default();
            if !crate::expr::evaluate(cond_expr, &existing) {
                return dynamodb_error_response(&conditional_check_failed());
            }
        }

        match self
            .backend
            .put_item(account_id, region, table_name, item)
            .await
        {
            Ok(old_item) => {
                let attributes = if return_values == "ALL_OLD" {
                    old_item.map(|old| convert_item_to_wire(&old))
                } else {
                    None
                };
                let output = wire::PutItemOutput {
                    attributes,
                    ..Default::default()
                };
                wire::serialize_put_item_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_get_item(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_get_item_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        if input.key.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing Key",
            );
        }
        let table_name = input.table_name;
        let key: Item = item_from_wire(input.key);

        let expr_names = input.expression_attribute_names.unwrap_or_default();
        let projection = crate::expr::parse_projection_expression(
            input.projection_expression.as_deref(),
            &expr_names,
        );

        match self
            .backend
            .get_item(account_id, region, table_name, key.clone())
            .await
        {
            Ok(Some(item)) => {
                let projected = match &projection {
                    Some(paths) => crate::expr::apply_projection(&item, paths),
                    None => item,
                };
                let output = wire::GetItemOutput {
                    item: Some(convert_item_to_wire(&projected)),
                    ..Default::default()
                };
                wire::serialize_get_item_response(&output)
            }
            Ok(None) => {
                let output = wire::GetItemOutput::default();
                wire::serialize_get_item_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_delete_item(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_item_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        if input.key.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing Key",
            );
        }
        let table_name = input.table_name;
        let key: Item = item_from_wire(input.key);
        let return_values = input.return_values.as_deref().unwrap_or("NONE");

        // --- ConditionExpression ---
        let expr_names = input.expression_attribute_names.unwrap_or_default();
        let expr_values = attr_map_from_wire(input.expression_attribute_values);
        let cond = match input.condition_expression.as_deref() {
            Some(s) => match crate::expr::parse_expression(s, &expr_names, &expr_values) {
                Ok(e) => Some(e),
                Err(msg) => {
                    return json_error_response(
                        400,
                        "com.amazonaws.dynamodb.v20120810#ValidationException",
                        &msg,
                    );
                }
            },
            None => None,
        };
        if let Some(ref cond_expr) = cond {
            let existing = self
                .backend
                .get_item(
                    account_id.clone(),
                    region.clone(),
                    table_name.clone(),
                    key.clone(),
                )
                .await
                .ok()
                .flatten()
                .unwrap_or_default();
            if !crate::expr::evaluate(cond_expr, &existing) {
                return dynamodb_error_response(&conditional_check_failed());
            }
        }

        match self
            .backend
            .delete_item(account_id, region, table_name, key.clone())
            .await
        {
            Ok(old_item) => {
                let attributes = if return_values == "ALL_OLD" {
                    old_item.map(|old| convert_item_to_wire(&old))
                } else {
                    None
                };
                let output = wire::DeleteItemOutput {
                    attributes,
                    ..Default::default()
                };
                wire::serialize_delete_item_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_query(&self, body: &[u8], account_id: String, region: String) -> MockResponse {
        let input = match wire::deserialize_query_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        let table_name = input.table_name;

        let limit = input.limit.map(|v| v as usize);
        let scan_index_forward = input.scan_index_forward.unwrap_or(true);

        let exclusive_start_key: Option<Item> = input.exclusive_start_key.map(item_from_wire);

        let index_name = input.index_name;

        let expr_names = input.expression_attribute_names.unwrap_or_default();
        let expr_values = attr_map_from_wire(input.expression_attribute_values);

        // Extract key conditions from KeyConditionExpression. The hash-key
        // (and optional sort-key) equality conditions go into `key_conditions`;
        // any sort-key range / `begins_with` predicate is returned separately
        // and applied by the backend before pagination.
        //
        // Falls back to the legacy `KeyConditions` v1 map when no expression
        // was supplied, matching the previous implementation.
        let (key_conditions, sort_key_condition) = match input.key_condition_expression.as_deref() {
            Some(expr_str) => {
                if let Ok(parsed) =
                    crate::expr::parse_expression(expr_str, &expr_names, &expr_values)
                {
                    let mut equalities = Item::new();
                    let mut sort_condition: Option<SortKeyCondition> = None;
                    walk_key_condition(parsed, &mut equalities, &mut sort_condition);
                    (equalities, sort_condition)
                } else {
                    // Legacy permissive split-on-AND fallback. The previous
                    // implementation also fell back here when the strict
                    // parser rejected an input.
                    let mut conditions = Item::new();
                    if !expr_values.is_empty() {
                        for part in expr_str.split(" AND ").chain(expr_str.split(" and ")) {
                            let part = part.trim();
                            if let Some((lhs, rhs)) = part.split_once('=') {
                                let lhs = lhs.trim();
                                let rhs = rhs.trim();
                                let attr_name = if let Some(alias) = lhs.strip_prefix('#') {
                                    expr_names
                                        .get(&format!("#{alias}"))
                                        .map(String::as_str)
                                        .unwrap_or(lhs)
                                } else {
                                    lhs
                                };
                                if rhs.starts_with(':')
                                    && let Some(val) = expr_values.get(rhs)
                                {
                                    conditions.insert(attr_name.to_string(), val.clone());
                                }
                            }
                        }
                    }
                    (conditions, None)
                }
            }
            None => {
                // v1 KeyConditions API: map of attr -> Condition. Convert
                // EQ comparisons into equality predicates.
                let mut equalities = Item::new();
                if let Some(kc) = input.key_conditions {
                    for (attr, cond) in kc {
                        if cond.comparison_operator == "EQ"
                            && let Some(values) = cond.attribute_value_list
                            && let Some(first) = values.into_iter().next()
                        {
                            equalities.insert(attr, AttributeValue::from(first));
                        }
                    }
                }
                (equalities, None)
            }
        };

        // --- FilterExpression ---
        let filter = match input.filter_expression.as_deref() {
            Some(s) => match crate::expr::parse_expression(s, &expr_names, &expr_values) {
                Ok(e) => Some(e),
                Err(msg) => {
                    return json_error_response(
                        400,
                        "com.amazonaws.dynamodb.v20120810#ValidationException",
                        &msg,
                    );
                }
            },
            None => None,
        };

        let projection = crate::expr::parse_projection_expression(
            input.projection_expression.as_deref(),
            &expr_names,
        );

        match self
            .backend
            .query(
                account_id,
                region,
                table_name,
                key_conditions.clone(),
                sort_key_condition,
                limit,
                scan_index_forward,
                exclusive_start_key.clone(),
                index_name,
            )
            .await
        {
            Ok(result) => {
                let scanned_count = result.items.len();
                let filtered_items: Vec<Item> = match filter {
                    Some(ref f) => result
                        .items
                        .into_iter()
                        .filter(|i| crate::expr::evaluate(f, i))
                        .collect(),
                    None => result.items,
                };
                let projected_items: Vec<Item> = match &projection {
                    Some(paths) => filtered_items
                        .iter()
                        .map(|i| crate::expr::apply_projection(i, paths))
                        .collect(),
                    None => filtered_items.clone(),
                };
                let output = wire::QueryOutput {
                    items: Some(convert_items_to_wire(&projected_items)),
                    count: Some(filtered_items.len() as i32),
                    scanned_count: Some(scanned_count as i32),
                    last_evaluated_key: result
                        .last_evaluated_key
                        .as_ref()
                        .map(convert_item_to_wire),
                    ..Default::default()
                };
                wire::serialize_query_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_scan(&self, body: &[u8], account_id: String, region: String) -> MockResponse {
        let input = match wire::deserialize_scan_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        let table_name = input.table_name;

        let limit = input.limit.map(|v| v as usize);
        let exclusive_start_key: Option<Item> = input.exclusive_start_key.map(item_from_wire);

        // --- FilterExpression ---
        let expr_names = input.expression_attribute_names.unwrap_or_default();
        let expr_values = attr_map_from_wire(input.expression_attribute_values);
        let filter = match input.filter_expression.as_deref() {
            Some(s) => match crate::expr::parse_expression(s, &expr_names, &expr_values) {
                Ok(e) => Some(e),
                Err(msg) => {
                    return json_error_response(
                        400,
                        "com.amazonaws.dynamodb.v20120810#ValidationException",
                        &msg,
                    );
                }
            },
            None => None,
        };

        let projection = crate::expr::parse_projection_expression(
            input.projection_expression.as_deref(),
            &expr_names,
        );

        match self
            .backend
            .scan(
                account_id,
                region,
                table_name,
                limit,
                exclusive_start_key.clone(),
            )
            .await
        {
            Ok(result) => {
                let scanned_count = result.items.len();
                let filtered_items: Vec<Item> = match filter {
                    Some(ref f) => result
                        .items
                        .into_iter()
                        .filter(|i| crate::expr::evaluate(f, i))
                        .collect(),
                    None => result.items,
                };
                let projected_items: Vec<Item> = match &projection {
                    Some(paths) => filtered_items
                        .iter()
                        .map(|i| crate::expr::apply_projection(i, paths))
                        .collect(),
                    None => filtered_items.clone(),
                };
                let output = wire::ScanOutput {
                    items: Some(convert_items_to_wire(&projected_items)),
                    count: Some(filtered_items.len() as i32),
                    scanned_count: Some(scanned_count as i32),
                    last_evaluated_key: result
                        .last_evaluated_key
                        .as_ref()
                        .map(convert_item_to_wire),
                    ..Default::default()
                };
                wire::serialize_scan_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }
    async fn handle_list_tables(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tables_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        let limit = input.limit.map(|v| v as usize);
        let exclusive_start = input.exclusive_start_table_name.as_deref();

        let all_names = match self.backend.list_tables(account_id, region).await {
            Ok(names) => names,
            Err(e) => return dynamodb_error_response(&e),
        };

        // Apply ExclusiveStartTableName cursor
        let start_idx = if let Some(cursor) = exclusive_start {
            all_names
                .iter()
                .position(|n| n == cursor)
                .map(|i| i + 1)
                .unwrap_or(0)
        } else {
            0
        };
        let remaining: Vec<String> = all_names[start_idx..].to_vec();

        let (page, last_evaluated_table_name) = if let Some(lim) = limit {
            let page: Vec<String> = remaining.iter().take(lim).cloned().collect();
            let last = if remaining.len() > lim {
                page.last().cloned()
            } else {
                None
            };
            (page, last)
        } else {
            (remaining, None)
        };

        let output = wire::ListTablesOutput {
            table_names: Some(page),
            last_evaluated_table_name,
        };
        wire::serialize_list_tables_response(&output)
    }

    async fn handle_update_item(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_update_item_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        if input.key.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing Key",
            );
        }
        let table_name = input.table_name;
        let key: Item = item_from_wire(input.key);
        let return_values = input.return_values.as_deref().unwrap_or("NONE");

        let expr_names = input.expression_attribute_names.unwrap_or_default();
        let expr_values = attr_map_from_wire(input.expression_attribute_values);

        let update_expression = input.update_expression.unwrap_or_default();
        let actions =
            crate::expr::parse_update_expression(&update_expression, &expr_names, &expr_values);

        // --- ConditionExpression ---
        let cond = match input.condition_expression.as_deref() {
            Some(s) => match crate::expr::parse_expression(s, &expr_names, &expr_values) {
                Ok(e) => Some(e),
                Err(msg) => {
                    return json_error_response(
                        400,
                        "com.amazonaws.dynamodb.v20120810#ValidationException",
                        &msg,
                    );
                }
            },
            None => None,
        };
        if let Some(ref cond_expr) = cond {
            let existing = self
                .backend
                .get_item(
                    account_id.clone(),
                    region.clone(),
                    table_name.clone(),
                    key.clone(),
                )
                .await
                .ok()
                .flatten()
                .unwrap_or_default();
            if !crate::expr::evaluate(cond_expr, &existing) {
                return dynamodb_error_response(&conditional_check_failed());
            }
        }

        match self
            .backend
            .update_item(account_id, region, table_name, key.clone(), actions)
            .await
        {
            Ok(updated_item) => {
                let attributes = if return_values == "ALL_NEW" {
                    updated_item.map(|item| convert_item_to_wire(&item))
                } else {
                    None
                };
                let output = wire::UpdateItemOutput {
                    attributes,
                    ..Default::default()
                };
                wire::serialize_update_item_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_batch_write_item(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_write_item_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };

        for (table_name, requests) in input.request_items {
            let mut puts = Vec::new();
            let mut deletes = Vec::new();

            for req in requests {
                if let Some(put_req) = req.put_request {
                    let item: Item = item_from_wire(put_req.item);
                    puts.push(item);
                }
                if let Some(del_req) = req.delete_request {
                    let key: Item = item_from_wire(del_req.key);
                    deletes.push(key);
                }
            }

            if let Err(e) = self
                .backend
                .batch_write_item(
                    account_id.clone(),
                    region.clone(),
                    table_name,
                    puts,
                    deletes,
                )
                .await
            {
                return dynamodb_error_response(&e);
            }
        }

        let output = wire::BatchWriteItemOutput {
            unprocessed_items: Some(HashMap::new()),
            ..Default::default()
        };
        wire::serialize_batch_write_item_response(&output)
    }

    async fn handle_batch_get_item(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_item_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };

        let mut responses: HashMap<String, Vec<HashMap<String, wire::AttributeValue>>> =
            HashMap::new();

        for (table_name, request) in input.request_items {
            let parsed_keys: Vec<Item> = request.keys.into_iter().map(item_from_wire).collect();

            match self
                .backend
                .batch_get_item(
                    account_id.clone(),
                    region.clone(),
                    table_name.clone(),
                    parsed_keys,
                )
                .await
            {
                Ok(items) => {
                    responses.insert(table_name, convert_items_to_wire(&items));
                }
                Err(e) => return dynamodb_error_response(&e),
            }
        }

        let output = wire::BatchGetItemOutput {
            responses: Some(responses),
            unprocessed_keys: Some(HashMap::new()),
            ..Default::default()
        };
        wire::serialize_batch_get_item_response(&output)
    }

    // --- New operation handlers ---

    async fn handle_update_table(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_update_table_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        let table_name = input.table_name.clone();

        let billing_mode = input.billing_mode.clone();
        let provisioned = input
            .provisioned_throughput
            .as_ref()
            .map(|pt| ProvisionedThroughput {
                read_capacity_units: pt.read_capacity_units,
                write_capacity_units: pt.write_capacity_units,
            });

        if let Some(spec) = &input.stream_specification {
            let stream_enabled = spec.stream_enabled;
            let stream_view_type = spec.stream_view_type.clone();
            if let Err(e) = self
                .backend
                .update_stream_specification(
                    account_id.clone(),
                    region.clone(),
                    table_name.clone(),
                    stream_enabled,
                    stream_view_type,
                )
                .await
            {
                return dynamodb_error_response(&e);
            }
        }

        match self
            .backend
            .update_table(account_id, region, table_name, billing_mode, provisioned)
            .await
        {
            Ok(table) => {
                let output = wire::UpdateTableOutput {
                    table_description: Some(table_to_wire_description(&table)),
                };
                wire::serialize_update_table_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_create_backup(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_create_backup_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        if input.backup_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing BackupName",
            );
        }

        match self
            .backend
            .create_backup(account_id, region, input.table_name, input.backup_name)
            .await
        {
            Ok(backup) => {
                let output = wire::CreateBackupOutput {
                    backup_details: Some(wire::BackupDetails {
                        backup_arn: Some(backup.backup_arn),
                        backup_name: Some(backup.backup_name),
                        backup_status: Some(backup.backup_status),
                        backup_type: Some(backup.backup_type),
                        backup_creation_date_time: Some(
                            backup.backup_creation_date_time.timestamp() as f64,
                        ),
                        backup_size_bytes: Some(backup.backup_size_bytes),
                        ..Default::default()
                    }),
                };
                wire::serialize_create_backup_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_delete_backup(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_backup_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.backup_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing BackupArn",
            );
        }

        match self
            .backend
            .delete_backup(account_id, region, input.backup_arn)
            .await
        {
            Ok(backup) => {
                let output = wire::DeleteBackupOutput {
                    backup_description: Some(backup_to_wire_description(&backup)),
                };
                wire::serialize_delete_backup_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_describe_backup(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_backup_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.backup_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing BackupArn",
            );
        }

        match self
            .backend
            .describe_backup(account_id, region, input.backup_arn)
            .await
        {
            Ok(backup) => {
                let output = wire::DescribeBackupOutput {
                    backup_description: Some(backup_to_wire_description(&backup)),
                };
                wire::serialize_describe_backup_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_list_backups(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_backups_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        let table_name = input.table_name.clone();

        let backups = match self
            .backend
            .list_backups(account_id, region, table_name)
            .await
        {
            Ok(b) => b,
            Err(e) => return dynamodb_error_response(&e),
        };
        let summaries: Vec<wire::BackupSummary> = backups
            .iter()
            .map(|b| wire::BackupSummary {
                backup_arn: Some(b.backup_arn.clone()),
                backup_name: Some(b.backup_name.clone()),
                backup_status: Some(b.backup_status.clone()),
                backup_type: Some(b.backup_type.clone()),
                backup_creation_date_time: Some(b.backup_creation_date_time.timestamp() as f64),
                backup_size_bytes: Some(b.backup_size_bytes),
                table_name: Some(b.table_name.clone()),
                table_arn: Some(b.table_arn.clone()),
                table_id: Some(b.table_id.clone()),
                ..Default::default()
            })
            .collect();

        let output = wire::ListBackupsOutput {
            backup_summaries: Some(summaries),
            ..Default::default()
        };
        wire::serialize_list_backups_response(&output)
    }

    async fn handle_restore_table_from_backup(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_table_from_backup_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.target_table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TargetTableName",
            );
        }
        if input.backup_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing BackupArn",
            );
        }
        let backup_arn = input.backup_arn.clone();
        let target_table_name = input.target_table_name.clone();

        // Get backup info before the restore (for RestoreSummary)
        let backup_table_arn = self
            .backend
            .describe_backup(account_id.clone(), region.clone(), backup_arn.clone())
            .await
            .ok()
            .map(|b| b.table_arn);
        match self
            .backend
            .restore_table_from_backup(account_id, region, backup_arn.clone(), target_table_name)
            .await
        {
            Ok(table) => {
                let mut desc = table_to_wire_description(&table);
                desc.restore_summary = Some(wire::RestoreSummary {
                    source_backup_arn: Some(backup_arn),
                    source_table_arn: backup_table_arn,
                    restore_date_time: Some(chrono::Utc::now().timestamp() as f64),
                    restore_in_progress: Some(false),
                });
                let output = wire::RestoreTableFromBackupOutput {
                    table_description: Some(desc),
                };
                wire::serialize_restore_table_from_backup_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_restore_table_to_point_in_time(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_table_to_point_in_time_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        let source_table_name = match input.source_table_name.as_deref() {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    "Missing SourceTableName",
                );
            }
        };
        if input.target_table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TargetTableName",
            );
        }
        let target_table_name = input.target_table_name.clone();

        // Get source table ARN before the restore (for RestoreSummary)
        let source_table_arn = self
            .backend
            .describe_table(
                account_id.clone(),
                region.clone(),
                source_table_name.clone(),
            )
            .await
            .ok()
            .map(|t| t.arn);
        match self
            .backend
            .restore_table_to_point_in_time(
                account_id,
                region,
                source_table_name,
                target_table_name,
            )
            .await
        {
            Ok(table) => {
                let mut desc = table_to_wire_description(&table);
                desc.restore_summary = Some(wire::RestoreSummary {
                    source_table_arn,
                    restore_date_time: Some(chrono::Utc::now().timestamp() as f64),
                    restore_in_progress: Some(false),
                    ..Default::default()
                });
                let output = wire::RestoreTableToPointInTimeOutput {
                    table_description: Some(desc),
                };
                wire::serialize_restore_table_to_point_in_time_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing ResourceArn",
            );
        }

        let tags: Vec<DynamoDbTag> = input
            .tags
            .into_iter()
            .map(|t| DynamoDbTag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let _ = self
            .backend
            .tag_resource(account_id, region, input.resource_arn, tags)
            .await;
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing ResourceArn",
            );
        }

        let _ = self
            .backend
            .untag_resource(account_id, region, input.resource_arn, input.tag_keys)
            .await;
        wire::serialize_untag_resource_response()
    }

    async fn handle_list_tags_of_resource(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_of_resource_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing ResourceArn",
            );
        }

        let tags = match self
            .backend
            .list_tags_of_resource(account_id, region, input.resource_arn)
            .await
        {
            Ok(t) => t,
            Err(e) => return dynamodb_error_response(&e),
        };
        let wire_tags: Vec<wire::Tag> = tags
            .iter()
            .map(|t| wire::Tag {
                key: t.key.clone(),
                value: t.value.clone(),
            })
            .collect();

        let output = wire::ListTagsOfResourceOutput {
            tags: Some(wire_tags),
            ..Default::default()
        };
        wire::serialize_list_tags_of_resource_response(&output)
    }

    async fn handle_update_time_to_live(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_update_time_to_live_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }

        // The typed input always materialises a TimeToLiveSpecification (with
        // serde defaults), so treat an entirely-default value as "absent" to
        // match the pre-migration semantics.
        let spec = &input.time_to_live_specification;
        if spec.attribute_name.is_empty() && !spec.enabled {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TimeToLiveSpecification",
            );
        }

        let attribute_name = spec.attribute_name.clone();
        let enabled = spec.enabled;

        match self
            .backend
            .update_time_to_live(
                account_id,
                region,
                input.table_name,
                attribute_name,
                enabled,
            )
            .await
        {
            Ok(config) => {
                let output = wire::UpdateTimeToLiveOutput {
                    time_to_live_specification: Some(wire::TimeToLiveSpecification {
                        attribute_name: config.attribute_name,
                        enabled: config.enabled,
                    }),
                };
                wire::serialize_update_time_to_live_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_describe_time_to_live(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_time_to_live_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }

        match self
            .backend
            .describe_time_to_live(account_id, region, input.table_name)
            .await
        {
            Ok(Some(config)) => {
                let status = if config.enabled {
                    "ENABLED"
                } else {
                    "DISABLED"
                };
                let output = wire::DescribeTimeToLiveOutput {
                    time_to_live_description: Some(wire::TimeToLiveDescription {
                        attribute_name: Some(config.attribute_name.clone()),
                        time_to_live_status: Some(status.to_string()),
                    }),
                };
                wire::serialize_describe_time_to_live_response(&output)
            }
            Ok(None) => {
                let output = wire::DescribeTimeToLiveOutput {
                    time_to_live_description: Some(wire::TimeToLiveDescription {
                        time_to_live_status: Some("DISABLED".to_string()),
                        ..Default::default()
                    }),
                };
                wire::serialize_describe_time_to_live_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_update_continuous_backups(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_update_continuous_backups_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }

        let enabled = input
            .point_in_time_recovery_specification
            .point_in_time_recovery_enabled;

        match self
            .backend
            .update_continuous_backups(account_id, region, input.table_name, enabled)
            .await
        {
            Ok(config) => {
                let pitr_status = if config.point_in_time_recovery_enabled {
                    "ENABLED"
                } else {
                    "DISABLED"
                };
                let output = wire::UpdateContinuousBackupsOutput {
                    continuous_backups_description: Some(wire::ContinuousBackupsDescription {
                        continuous_backups_status: Some("ENABLED".to_string()),
                        point_in_time_recovery_description: Some(
                            wire::PointInTimeRecoveryDescription {
                                point_in_time_recovery_status: Some(pitr_status.to_string()),
                                ..Default::default()
                            },
                        ),
                    }),
                };
                wire::serialize_update_continuous_backups_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_describe_continuous_backups(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_continuous_backups_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }

        match self
            .backend
            .describe_continuous_backups(account_id, region, input.table_name)
            .await
        {
            Ok(config) => {
                let pitr_status = config
                    .map(|c| {
                        if c.point_in_time_recovery_enabled {
                            "ENABLED"
                        } else {
                            "DISABLED"
                        }
                    })
                    .unwrap_or("DISABLED");

                let output = wire::DescribeContinuousBackupsOutput {
                    continuous_backups_description: Some(wire::ContinuousBackupsDescription {
                        continuous_backups_status: Some("ENABLED".to_string()),
                        point_in_time_recovery_description: Some(
                            wire::PointInTimeRecoveryDescription {
                                point_in_time_recovery_status: Some(pitr_status.to_string()),
                                ..Default::default()
                            },
                        ),
                    }),
                };
                wire::serialize_describe_continuous_backups_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_put_resource_policy(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing ResourceArn",
            );
        }
        if input.policy.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing Policy",
            );
        }

        match self
            .backend
            .put_resource_policy(account_id, region, input.resource_arn, input.policy)
            .await
        {
            Ok(revision_id) => {
                let output = wire::PutResourcePolicyOutput {
                    revision_id: Some(revision_id),
                };
                wire::serialize_put_resource_policy_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_get_resource_policy(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing ResourceArn",
            );
        }

        match self
            .backend
            .get_resource_policy(account_id, region, input.resource_arn)
            .await
        {
            Ok(Some(rp)) => {
                let output = wire::GetResourcePolicyOutput {
                    policy: Some(rp.policy.clone()),
                    revision_id: Some(rp.revision_id.clone()),
                };
                wire::serialize_get_resource_policy_response(&output)
            }
            Ok(None) => {
                let output = wire::GetResourcePolicyOutput::default();
                wire::serialize_get_resource_policy_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing ResourceArn",
            );
        }

        match self
            .backend
            .delete_resource_policy(account_id, region, input.resource_arn)
            .await
        {
            Ok(revision_id) => {
                let output = wire::DeleteResourcePolicyOutput {
                    revision_id: Some(revision_id),
                };
                wire::serialize_delete_resource_policy_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_describe_endpoints(&self, region: &str) -> MockResponse {
        let output = wire::DescribeEndpointsResponse {
            endpoints: Some(vec![wire::Endpoint {
                address: Some(format!("dynamodb.{region}.amazonaws.com")),
                cache_period_in_minutes: Some(1440),
            }]),
        };
        wire::serialize_describe_endpoints_response(&output)
    }

    async fn handle_transact_get_items(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_transact_get_items_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };

        let keys: Vec<(String, Item)> = input
            .transact_items
            .into_iter()
            .map(|t| (t.get.table_name, item_from_wire(t.get.key)))
            .collect();

        match self
            .backend
            .transact_get_items(account_id, region, keys)
            .await
        {
            Ok(items) => {
                let responses: Vec<wire::ItemResponse> = items
                    .into_iter()
                    .map(|opt_item| wire::ItemResponse {
                        item: opt_item.map(|item| convert_item_to_wire(&item)),
                    })
                    .collect();

                let output = wire::TransactGetItemsOutput {
                    responses: Some(responses),
                    ..Default::default()
                };
                wire::serialize_transact_get_items_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_transact_write_items(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_transact_write_items_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        let transact_items = input.transact_items;

        let mut puts: Vec<(String, Item)> = Vec::new();
        let mut deletes: Vec<(String, Item)> = Vec::new();
        let mut updates: Vec<(String, Item, Vec<crate::types::UpdateAction>)> = Vec::new();

        // Track cancellation reasons (one per transact item, in order).
        let item_count = transact_items.len();
        let mut cancellation_reasons: Vec<Option<String>> = vec![None; item_count];
        let mut any_cancelled = false;

        // A normalised view of one transact-item's sub-body: the wire types
        // for Put/Delete/Update/ConditionCheck have similar but not identical
        // fields, so we reduce them all to (op, table_name, key/item,
        // condition_expression, expr_names, expr_values).
        struct SubBody {
            op: &'static str,
            table_name: String,
            // For Put: this is the full item; for the others, the primary key.
            key_or_item: Item,
            condition_expression: Option<String>,
            expression_attribute_names: HashMap<String, String>,
            expression_attribute_values: HashMap<String, AttributeValue>,
        }

        for (idx, t) in transact_items.into_iter().enumerate() {
            let (sub_body, update_for_action): (Option<SubBody>, Option<crate::model::Update>) =
                if let Some(put) = t.put {
                    (
                        Some(SubBody {
                            op: "Put",
                            table_name: put.table_name,
                            key_or_item: item_from_wire(put.item),
                            condition_expression: put.condition_expression,
                            expression_attribute_names: put
                                .expression_attribute_names
                                .unwrap_or_default(),
                            expression_attribute_values: attr_map_from_wire(
                                put.expression_attribute_values,
                            ),
                        }),
                        None,
                    )
                } else if let Some(del) = t.delete {
                    (
                        Some(SubBody {
                            op: "Delete",
                            table_name: del.table_name,
                            key_or_item: item_from_wire(del.key),
                            condition_expression: del.condition_expression,
                            expression_attribute_names: del
                                .expression_attribute_names
                                .unwrap_or_default(),
                            expression_attribute_values: attr_map_from_wire(
                                del.expression_attribute_values,
                            ),
                        }),
                        None,
                    )
                } else if let Some(upd) = t.update {
                    (
                        Some(SubBody {
                            op: "Update",
                            table_name: upd.table_name.clone(),
                            key_or_item: item_from_wire(upd.key.clone()),
                            condition_expression: upd.condition_expression.clone(),
                            expression_attribute_names: upd
                                .expression_attribute_names
                                .clone()
                                .unwrap_or_default(),
                            expression_attribute_values: attr_map_from_wire(
                                upd.expression_attribute_values.clone(),
                            ),
                        }),
                        Some(upd),
                    )
                } else if let Some(chk) = t.condition_check {
                    (
                        Some(SubBody {
                            op: "ConditionCheck",
                            table_name: chk.table_name,
                            key_or_item: item_from_wire(chk.key),
                            condition_expression: Some(chk.condition_expression),
                            expression_attribute_names: chk
                                .expression_attribute_names
                                .unwrap_or_default(),
                            expression_attribute_values: attr_map_from_wire(
                                chk.expression_attribute_values,
                            ),
                        }),
                        None,
                    )
                } else {
                    (None, None)
                };

            if let Some(sub) = sub_body {
                // Parse and evaluate ConditionExpression if present
                let cond = match sub.condition_expression.as_deref() {
                    Some(s) => match crate::expr::parse_expression(
                        s,
                        &sub.expression_attribute_names,
                        &sub.expression_attribute_values,
                    ) {
                        Ok(e) => Some(e),
                        Err(msg) => {
                            return json_error_response(
                                400,
                                "com.amazonaws.dynamodb.v20120810#ValidationException",
                                &msg,
                            );
                        }
                    },
                    None => None,
                };

                if let Some(ref cond_expr) = cond {
                    // For Put, the existing item key needs deriving from the
                    // table schema; the simple fallback here is to use the
                    // full item as the key search (the backend's
                    // build_key_for_item would be ideal but we don't have
                    // access to state inside dispatch). The original code
                    // similarly passed the full Put item as the key value.
                    let key_for_lookup: Item = sub.key_or_item.clone();
                    let existing = self
                        .backend
                        .get_item(
                            account_id.clone(),
                            region.clone(),
                            sub.table_name.clone(),
                            key_for_lookup,
                        )
                        .await
                        .ok()
                        .flatten()
                        .unwrap_or_default();
                    if !crate::expr::evaluate(cond_expr, &existing) {
                        cancellation_reasons[idx] = Some("ConditionalCheckFailed".to_string());
                        any_cancelled = true;
                    }
                }

                // Collect the operation (skip ConditionCheck — it's check-only)
                match sub.op {
                    "Put" => {
                        puts.push((sub.table_name, sub.key_or_item));
                    }
                    "Delete" => {
                        deletes.push((sub.table_name, sub.key_or_item));
                    }
                    "Update" => {
                        let upd = update_for_action.expect("Update branch sets update_for_action");
                        let upd_expr_names = upd.expression_attribute_names.unwrap_or_default();
                        let upd_expr_values = attr_map_from_wire(upd.expression_attribute_values);
                        let actions = crate::expr::parse_update_expression(
                            &upd.update_expression,
                            &upd_expr_names,
                            &upd_expr_values,
                        );
                        updates.push((sub.table_name, sub.key_or_item, actions));
                    }
                    _ => {} // ConditionCheck — no mutation
                }
            }
        }

        // If any condition failed, cancel the entire transaction
        if any_cancelled {
            let reasons: Vec<Value> = cancellation_reasons
                .iter()
                .map(|r| match r {
                    Some(code) => json!({
                        "Code": code,
                        "Message": "The conditional request failed",
                    }),
                    None => json!({ "Code": "None" }),
                })
                .collect();
            let body = json!({
                "__type": "com.amazonaws.dynamodb.v20120810#TransactionCanceledException",
                "message": "Transaction cancelled, please refer cancellation reasons for specific reasons [ConditionalCheckFailed]",
                "CancellationReasons": reasons,
            });
            return MockResponse::json(400, body.to_string());
        }

        match self
            .backend
            .transact_write_items(account_id, region, puts, deletes, updates)
            .await
        {
            Ok(()) => {
                let output = wire::TransactWriteItemsOutput::default();
                wire::serialize_transact_write_items_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_batch_execute_statement(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_execute_statement_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };

        let mut responses: Vec<wire::BatchStatementResponse> = Vec::new();

        for stmt in input.statements {
            let statement = stmt.statement.as_str();
            let parameters: Vec<Value> = wire_attrs_to_json(stmt.parameters);

            let op = match winterbaume_partiql::parse_statement(statement, &parameters) {
                Ok(op) => op,
                Err(e) => {
                    responses.push(wire::BatchStatementResponse {
                        error: Some(wire::BatchStatementError {
                            code: Some("ValidationError".to_string()),
                            message: Some(e.to_string()),
                            item: None,
                        }),
                        ..Default::default()
                    });
                    continue;
                }
            };

            // EXISTS is transactions-only; reject in BatchExecuteStatement.
            // Wording matches AWS's exact response, verified empirically
            // against real DDB in `ap-northeast-1` on 2026-04-29 — same
            // message the single-statement path returns.
            if matches!(op, winterbaume_partiql::DdbOperation::Exists(_)) {
                responses.push(wire::BatchStatementResponse {
                    error: Some(wire::BatchStatementError {
                        code: Some("ValidationError".to_string()),
                        message: Some(
                            "EXISTS can only be used in ExecuteTransaction write requests.".into(),
                        ),
                        item: None,
                    }),
                    ..Default::default()
                });
                continue;
            }

            // Reject arithmetic / unary-neg in WHERE-clause operands with
            // the AWS-shaped wording before running the IR.
            if let Err(msg) = validate_partiql_op(&op) {
                responses.push(wire::BatchStatementResponse {
                    error: Some(wire::BatchStatementError {
                        code: Some("ValidationError".to_string()),
                        message: Some(msg),
                        item: None,
                    }),
                    ..Default::default()
                });
                continue;
            }

            match crate::partiql_exec::execute_partiql_via_backend(
                Arc::clone(&self.backend),
                account_id.clone(),
                region.clone(),
                op.clone(),
                true,
            )
            .await
            {
                Ok(crate::partiql_exec::PartiqlResult::Items(items)) => {
                    let item = items.into_iter().next().map(|item| {
                        item.into_iter()
                            .map(|(k, v)| (k, value_to_wire_attribute_value(&v)))
                            .collect()
                    });
                    responses.push(wire::BatchStatementResponse {
                        item,
                        table_name: Some(op.table_name().to_string()),
                        ..Default::default()
                    });
                }
                Ok(crate::partiql_exec::PartiqlResult::Empty) => {
                    responses.push(wire::BatchStatementResponse {
                        table_name: Some(op.table_name().to_string()),
                        ..Default::default()
                    });
                }
                Err(e) => {
                    let et = dynamodb_error_type(&e);
                    responses.push(wire::BatchStatementResponse {
                        error: Some(wire::BatchStatementError {
                            code: Some(et.rsplit('#').next().unwrap_or(et).to_string()),
                            message: Some(e.to_string()),
                            item: None,
                        }),
                        ..Default::default()
                    });
                }
            }
        }

        let output = wire::BatchExecuteStatementOutput {
            responses: Some(responses),
            ..Default::default()
        };
        wire::serialize_batch_execute_statement_response(&output)
    }

    async fn handle_execute_statement(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_execute_statement_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        let statement = input.statement.as_str();
        let parameters: Vec<Value> = wire_attrs_to_json(input.parameters);

        let op = match winterbaume_partiql::parse_statement(statement, &parameters) {
            Ok(op) => op,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e.to_string(),
                );
            }
        };

        // EXISTS is documented as transactions-only:
        // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ql-functions.exists.html
        // The exact error string here matches what real AWS DynamoDB
        // returns for `EXISTS(...)` submitted to ExecuteStatement, verified
        // against `ap-northeast-1` on 2026-04-29.
        if matches!(op, winterbaume_partiql::DdbOperation::Exists(_)) {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "EXISTS can only be used in ExecuteTransaction write requests.",
            );
        }

        // Reject arithmetic / unary-neg in WHERE-clause operands with the
        // AWS-shaped wording before running the IR.
        if let Err(msg) = validate_partiql_op(&op) {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                &msg,
            );
        }

        match crate::partiql_exec::execute_partiql_via_backend(
            Arc::clone(&self.backend),
            account_id,
            region,
            op,
            false,
        )
        .await
        {
            Ok(crate::partiql_exec::PartiqlResult::Items(items)) => {
                let wire_items: Vec<HashMap<String, wire::AttributeValue>> = items
                    .into_iter()
                    .map(|item| {
                        item.into_iter()
                            .map(|(k, v)| (k, value_to_wire_attribute_value(&v)))
                            .collect()
                    })
                    .collect();
                let output = wire::ExecuteStatementOutput {
                    items: Some(wire_items),
                    ..Default::default()
                };
                wire::serialize_execute_statement_response(&output)
            }
            Ok(crate::partiql_exec::PartiqlResult::Empty) => {
                let output = wire::ExecuteStatementOutput {
                    items: Some(vec![]),
                    ..Default::default()
                };
                wire::serialize_execute_statement_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_execute_transaction(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_execute_transaction_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };

        let mut responses: Vec<wire::ItemResponse> = Vec::new();

        for stmt in input.transact_statements {
            let statement = stmt.statement.as_str();
            let parameters: Vec<Value> = wire_attrs_to_json(stmt.parameters);

            let op = match winterbaume_partiql::parse_statement(statement, &parameters) {
                Ok(op) => op,
                Err(e) => {
                    return json_error_response(
                        400,
                        "com.amazonaws.dynamodb.v20120810#ValidationException",
                        &e.to_string(),
                    );
                }
            };

            // Reject arithmetic / unary-neg in WHERE-clause operands with
            // the AWS-shaped wording before running the IR. Applies to the
            // inner SELECT of an EXISTS() too.
            if let Err(msg) = validate_partiql_op(&op) {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &msg,
                );
            }

            // EXISTS is a condition check: TRUE iff the inner SELECT yields
            // any items. Empty result aborts the whole transaction with
            // TransactionCanceledException.
            let is_exists = matches!(op, winterbaume_partiql::DdbOperation::Exists(_));

            // AWS imposes two runtime constraints on the inner SELECT of
            // EXISTS, verified empirically against real DynamoDB on
            // 2026-04-29: (1) the WHERE must specify the full primary key
            // by equality, and (2) it must include at least one non-key
            // predicate. We enforce both here so MockAws callers see the
            // same ValidationException shape as real DDB.
            if let winterbaume_partiql::DdbOperation::Exists(inner_sel) = &op {
                let table_name = inner_sel.table_name.clone();
                let table = match self
                    .backend
                    .describe_table(account_id.clone(), region.clone(), table_name)
                    .await
                {
                    Ok(t) => t,
                    Err(e) => return dynamodb_error_response(&e),
                };
                if let Err(msg) = validate_exists_where(
                    inner_sel.where_clause.as_deref().unwrap_or(&[]),
                    &table.hash_key_attr,
                    table.range_key_attr.as_deref(),
                ) {
                    return json_error_response(
                        400,
                        "com.amazonaws.dynamodb.v20120810#ValidationException",
                        &msg,
                    );
                }
            }

            match crate::partiql_exec::execute_partiql_via_backend(
                Arc::clone(&self.backend),
                account_id.clone(),
                region.clone(),
                op,
                true,
            )
            .await
            {
                Ok(crate::partiql_exec::PartiqlResult::Items(items)) => {
                    if is_exists && items.is_empty() {
                        return json_error_response(
                            400,
                            "com.amazonaws.dynamodb.v20120810#TransactionCanceledException",
                            "Transaction cancelled, EXISTS condition not met",
                        );
                    }
                    let item = items.into_iter().next().map(|item| {
                        item.into_iter()
                            .map(|(k, v)| (k, value_to_wire_attribute_value(&v)))
                            .collect()
                    });
                    responses.push(wire::ItemResponse { item });
                }
                Ok(crate::partiql_exec::PartiqlResult::Empty) => {
                    if is_exists {
                        return json_error_response(
                            400,
                            "com.amazonaws.dynamodb.v20120810#TransactionCanceledException",
                            "Transaction cancelled, EXISTS condition not met",
                        );
                    }
                    responses.push(wire::ItemResponse::default());
                }
                Err(e) => {
                    return dynamodb_error_response(&e);
                }
            }
        }

        let output = wire::ExecuteTransactionOutput {
            responses: Some(responses),
            ..Default::default()
        };
        wire::serialize_execute_transaction_response(&output)
    }

    async fn handle_describe_export(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_export_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.export_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing ExportArn",
            );
        }

        match self
            .backend
            .describe_export(account_id, region, input.export_arn)
            .await
        {
            Ok(info) => {
                let output = wire::DescribeExportOutput {
                    export_description: Some(wire::ExportDescription {
                        export_arn: Some(info.export_arn),
                        table_arn: Some(info.table_arn),
                        export_status: Some(info.export_status),
                        start_time: Some(info.start_time.timestamp() as f64),
                        export_format: Some(info.export_format),
                        s3_bucket: info.s3_bucket,
                        s3_prefix: info.s3_prefix,
                        ..Default::default()
                    }),
                };
                wire::serialize_describe_export_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_describe_import(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_import_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.import_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing ImportArn",
            );
        }

        match self
            .backend
            .describe_import(account_id, region, input.import_arn)
            .await
        {
            Ok(info) => {
                let output = wire::DescribeImportOutput {
                    import_table_description: Some(wire::ImportTableDescription {
                        import_arn: Some(info.import_arn),
                        import_status: Some(info.import_status),
                        table_arn: info.table_arn,
                        ..Default::default()
                    }),
                };
                wire::serialize_describe_import_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_import_table(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_import_table_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        let table_name = if input.table_creation_parameters.table_name.is_empty() {
            "imported-table".to_string()
        } else {
            input.table_creation_parameters.table_name.clone()
        };
        let s3_bucket_source = {
            let bucket = input.s3_bucket_source.s3_bucket.clone();
            if bucket.is_empty() {
                None
            } else {
                Some(bucket)
            }
        };
        let input_format = if input.input_format.is_empty() {
            None
        } else {
            Some(input.input_format.clone())
        };

        let info = self
            .backend
            .import_table(
                account_id,
                region,
                table_name,
                s3_bucket_source,
                input_format,
            )
            .await;

        let output = wire::ImportTableOutput {
            import_table_description: Some(wire::ImportTableDescription {
                import_arn: Some(info.import_arn),
                import_status: Some(info.import_status),
                table_arn: info.table_arn,
                ..Default::default()
            }),
        };
        wire::serialize_import_table_response(&output)
    }

    async fn handle_list_exports(&self, account_id: String, region: String) -> MockResponse {
        let exports = self.backend.list_exports(account_id, region).await;
        let summaries: Vec<wire::ExportSummary> = exports
            .iter()
            .map(|e| wire::ExportSummary {
                export_arn: Some(e.export_arn.clone()),
                export_status: Some(e.export_status.clone()),
                ..Default::default()
            })
            .collect();
        let output = wire::ListExportsOutput {
            export_summaries: Some(summaries),
            ..Default::default()
        };
        wire::serialize_list_exports_response(&output)
    }

    // --- Global Tables (v1 API) ---

    async fn handle_create_global_table(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_create_global_table_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.global_table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing GlobalTableName",
            );
        }

        let replication_group: Vec<String> = input
            .replication_group
            .into_iter()
            .filter_map(|r| r.region_name)
            .collect();

        match self
            .backend
            .create_global_table(
                account_id,
                region,
                input.global_table_name,
                replication_group,
            )
            .await
        {
            Ok(gt) => {
                let desc = global_table_to_wire(&gt);
                let output = wire::CreateGlobalTableOutput {
                    global_table_description: Some(desc),
                };
                wire::serialize_create_global_table_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_describe_global_table(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_global_table_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.global_table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing GlobalTableName",
            );
        }

        match self
            .backend
            .describe_global_table(account_id, region, input.global_table_name)
            .await
        {
            Ok(gt) => {
                let desc = global_table_to_wire(&gt);
                let output = wire::DescribeGlobalTableOutput {
                    global_table_description: Some(desc),
                };
                wire::serialize_describe_global_table_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_describe_global_table_settings(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_global_table_settings_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.global_table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing GlobalTableName",
            );
        }
        let global_table_name = input.global_table_name.clone();

        match self
            .backend
            .describe_global_table(account_id, region.clone(), global_table_name.clone())
            .await
        {
            Ok(gt) => {
                let replica_settings: Vec<wire::ReplicaSettingsDescription> = gt
                    .replication_group
                    .iter()
                    .map(|r| wire::ReplicaSettingsDescription {
                        region_name: Some(r.clone()),
                        replica_status: Some("ACTIVE".to_string()),
                        ..Default::default()
                    })
                    .collect();
                // If no replicas explicitly listed, include current region
                let replica_settings = if replica_settings.is_empty() {
                    vec![wire::ReplicaSettingsDescription {
                        region_name: Some(region.to_string()),
                        replica_status: Some("ACTIVE".to_string()),
                        ..Default::default()
                    }]
                } else {
                    replica_settings
                };
                let output = wire::DescribeGlobalTableSettingsOutput {
                    global_table_name: Some(global_table_name),
                    replica_settings: Some(replica_settings),
                };
                wire::serialize_describe_global_table_settings_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_update_global_table(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_update_global_table_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.global_table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing GlobalTableName",
            );
        }

        let replicas_to_create: Vec<String> = input
            .replica_updates
            .iter()
            .filter_map(|u| u.create.as_ref().map(|c| c.region_name.clone()))
            .collect();

        let replicas_to_delete: Vec<String> = input
            .replica_updates
            .iter()
            .filter_map(|u| u.delete.as_ref().map(|d| d.region_name.clone()))
            .collect();

        match self
            .backend
            .update_global_table(
                account_id,
                region,
                input.global_table_name,
                replicas_to_create,
                replicas_to_delete,
            )
            .await
        {
            Ok(gt) => {
                let desc = global_table_to_wire(&gt);
                let output = wire::UpdateGlobalTableOutput {
                    global_table_description: Some(desc),
                };
                wire::serialize_update_global_table_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_update_global_table_settings(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_update_global_table_settings_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.global_table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing GlobalTableName",
            );
        }
        let global_table_name = input.global_table_name.clone();

        match self
            .backend
            .describe_global_table(account_id, region.clone(), global_table_name.clone())
            .await
        {
            Ok(gt) => {
                let replica_settings: Vec<wire::ReplicaSettingsDescription> = gt
                    .replication_group
                    .iter()
                    .map(|r| wire::ReplicaSettingsDescription {
                        region_name: Some(r.clone()),
                        replica_status: Some("ACTIVE".to_string()),
                        ..Default::default()
                    })
                    .collect();
                let replica_settings = if replica_settings.is_empty() {
                    vec![wire::ReplicaSettingsDescription {
                        region_name: Some(region.to_string()),
                        replica_status: Some("ACTIVE".to_string()),
                        ..Default::default()
                    }]
                } else {
                    replica_settings
                };
                let output = wire::UpdateGlobalTableSettingsOutput {
                    global_table_name: Some(global_table_name),
                    replica_settings: Some(replica_settings),
                };
                wire::serialize_update_global_table_settings_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_list_global_tables(&self, account_id: String, region: String) -> MockResponse {
        let gts = match self.backend.list_global_tables(account_id, region).await {
            Ok(g) => g,
            Err(e) => return dynamodb_error_response(&e),
        };
        let global_tables: Vec<wire::GlobalTable> = gts
            .iter()
            .map(|gt| wire::GlobalTable {
                global_table_name: Some(gt.global_table_name.clone()),
                replication_group: Some(
                    gt.replication_group
                        .iter()
                        .map(|r| wire::Replica {
                            region_name: Some(r.clone()),
                        })
                        .collect(),
                ),
            })
            .collect();
        let output = wire::ListGlobalTablesOutput {
            global_tables: Some(global_tables),
            ..Default::default()
        };
        wire::serialize_list_global_tables_response(&output)
    }

    // --- Kinesis Streaming Destinations ---

    async fn handle_enable_kinesis_streaming_destination(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_kinesis_streaming_destination_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        if input.stream_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing StreamArn",
            );
        }
        let precision = input
            .enable_kinesis_streaming_configuration
            .as_ref()
            .and_then(|c| c.approximate_creation_date_time_precision.clone());
        let table_name = input.table_name.clone();

        match self
            .backend
            .enable_kinesis_streaming_destination(
                account_id,
                region,
                table_name.clone(),
                input.stream_arn,
                precision,
            )
            .await
        {
            Ok(dest) => {
                let output = wire::KinesisStreamingDestinationOutput {
                    table_name: Some(table_name),
                    stream_arn: Some(dest.stream_arn),
                    destination_status: Some(dest.destination_status),
                    enable_kinesis_streaming_configuration: dest
                        .approximate_creation_date_time_precision
                        .map(|p| wire::EnableKinesisStreamingConfiguration {
                            approximate_creation_date_time_precision: Some(p),
                        }),
                };
                wire::serialize_enable_kinesis_streaming_destination_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_disable_kinesis_streaming_destination(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_kinesis_streaming_destination_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        if input.stream_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing StreamArn",
            );
        }
        let table_name = input.table_name.clone();

        match self
            .backend
            .disable_kinesis_streaming_destination(
                account_id,
                region,
                table_name.clone(),
                input.stream_arn,
            )
            .await
        {
            Ok(dest) => {
                let output = wire::KinesisStreamingDestinationOutput {
                    table_name: Some(table_name),
                    stream_arn: Some(dest.stream_arn),
                    destination_status: Some(dest.destination_status),
                    ..Default::default()
                };
                wire::serialize_disable_kinesis_streaming_destination_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_describe_kinesis_streaming_destination(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_kinesis_streaming_destination_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        let table_name = input.table_name.clone();

        match self
            .backend
            .describe_kinesis_streaming_destinations(account_id, region, table_name.clone())
            .await
        {
            Ok(dests) => {
                let kinesis_dests: Vec<wire::KinesisDataStreamDestination> = dests
                    .iter()
                    .map(|d| wire::KinesisDataStreamDestination {
                        stream_arn: Some(d.stream_arn.clone()),
                        destination_status: Some(d.destination_status.clone()),
                        approximate_creation_date_time_precision: d
                            .approximate_creation_date_time_precision
                            .clone(),
                        ..Default::default()
                    })
                    .collect();
                let output = wire::DescribeKinesisStreamingDestinationOutput {
                    table_name: Some(table_name),
                    kinesis_data_stream_destinations: Some(kinesis_dests),
                };
                wire::serialize_describe_kinesis_streaming_destination_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_update_kinesis_streaming_destination(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_update_kinesis_streaming_destination_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        if input.stream_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing StreamArn",
            );
        }
        let precision = input
            .update_kinesis_streaming_configuration
            .as_ref()
            .and_then(|c| c.approximate_creation_date_time_precision.clone());
        let table_name = input.table_name.clone();

        match self
            .backend
            .update_kinesis_streaming_destination(
                account_id,
                region,
                table_name.clone(),
                input.stream_arn,
                precision,
            )
            .await
        {
            Ok(dest) => {
                let output = wire::UpdateKinesisStreamingDestinationOutput {
                    table_name: Some(table_name),
                    stream_arn: Some(dest.stream_arn),
                    destination_status: Some(dest.destination_status),
                    update_kinesis_streaming_configuration: dest
                        .approximate_creation_date_time_precision
                        .map(|p| wire::UpdateKinesisStreamingConfiguration {
                            approximate_creation_date_time_precision: Some(p),
                        }),
                };
                wire::serialize_update_kinesis_streaming_destination_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    // --- Contributor Insights ---

    async fn handle_describe_contributor_insights(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_contributor_insights_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        let table_name = input.table_name.clone();
        let index_name = input.index_name.clone();

        match self
            .backend
            .describe_contributor_insights(
                account_id,
                region,
                table_name.clone(),
                index_name.clone(),
            )
            .await
        {
            Ok(Some(config)) => {
                let output = wire::DescribeContributorInsightsOutput {
                    table_name: Some(config.table_name.clone()),
                    index_name: config.index_name.clone(),
                    contributor_insights_status: Some(config.status.clone()),
                    contributor_insights_mode: Some(if config.status == "ENABLED" {
                        "ENABLED".to_string()
                    } else {
                        "DISABLED".to_string()
                    }),
                    last_update_date_time: Some(config.last_update_date_time.timestamp() as f64),
                    ..Default::default()
                };
                wire::serialize_describe_contributor_insights_response(&output)
            }
            Ok(None) => {
                // Not configured yet — return DISABLED
                let output = wire::DescribeContributorInsightsOutput {
                    table_name: Some(table_name),
                    index_name,
                    contributor_insights_status: Some("DISABLED".to_string()),
                    contributor_insights_mode: Some("DISABLED".to_string()),
                    ..Default::default()
                };
                wire::serialize_describe_contributor_insights_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_update_contributor_insights(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_update_contributor_insights_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        if input.contributor_insights_action.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing ContributorInsightsAction",
            );
        }
        let table_name = input.table_name.clone();
        let index_name = input.index_name.clone();
        let mode = input.contributor_insights_action.clone();

        match self
            .backend
            .update_contributor_insights(account_id, region, table_name, index_name, mode.clone())
            .await
        {
            Ok(config) => {
                let output = wire::UpdateContributorInsightsOutput {
                    table_name: Some(config.table_name),
                    index_name: config.index_name,
                    contributor_insights_status: Some(config.status),
                    contributor_insights_mode: Some(mode),
                };
                wire::serialize_update_contributor_insights_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_list_contributor_insights(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_list_contributor_insights_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        let table_name = input.table_name.clone();

        let configs = match self
            .backend
            .list_contributor_insights(account_id, region, table_name)
            .await
        {
            Ok(c) => c,
            Err(e) => return dynamodb_error_response(&e),
        };
        let summaries: Vec<wire::ContributorInsightsSummary> = configs
            .iter()
            .map(|c| wire::ContributorInsightsSummary {
                table_name: Some(c.table_name.clone()),
                index_name: c.index_name.clone(),
                contributor_insights_status: Some(c.status.clone()),
                contributor_insights_mode: Some(if c.status == "ENABLED" {
                    "ENABLED".to_string()
                } else {
                    "DISABLED".to_string()
                }),
            })
            .collect();
        let output = wire::ListContributorInsightsOutput {
            contributor_insights_summaries: Some(summaries),
            ..Default::default()
        };
        wire::serialize_list_contributor_insights_response(&output)
    }

    // --- Remaining operations ---

    async fn handle_describe_limits(&self) -> MockResponse {
        let output = wire::DescribeLimitsOutput {
            account_max_read_capacity_units: Some(80000),
            account_max_write_capacity_units: Some(80000),
            table_max_read_capacity_units: Some(40000),
            table_max_write_capacity_units: Some(40000),
        };
        wire::serialize_describe_limits_response(&output)
    }

    async fn handle_list_imports(&self, account_id: String, region: String) -> MockResponse {
        let imports = self.backend.list_imports(account_id, region).await;
        let summaries: Vec<wire::ImportSummary> = imports
            .iter()
            .map(|i| wire::ImportSummary {
                import_arn: Some(i.import_arn.clone()),
                import_status: Some(i.import_status.clone()),
                table_arn: i.table_arn.clone(),
                ..Default::default()
            })
            .collect();
        let output = wire::ListImportsOutput {
            import_summary_list: Some(summaries),
            ..Default::default()
        };
        wire::serialize_list_imports_response(&output)
    }

    async fn handle_export_table_to_point_in_time(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_export_table_to_point_in_time_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_arn.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableArn",
            );
        }
        let s3_bucket = input.s3_bucket.clone();
        let s3_prefix = input.s3_prefix.clone();
        let export_format = input.export_format.clone();

        match self
            .backend
            .export_table_to_point_in_time(
                account_id,
                region,
                input.table_arn,
                if s3_bucket.is_empty() {
                    None
                } else {
                    Some(s3_bucket)
                },
                s3_prefix,
                export_format,
            )
            .await
        {
            Ok(info) => {
                let output = wire::ExportTableToPointInTimeOutput {
                    export_description: Some(wire::ExportDescription {
                        export_arn: Some(info.export_arn),
                        table_arn: Some(info.table_arn),
                        export_status: Some(info.export_status),
                        start_time: Some(info.start_time.timestamp() as f64),
                        export_format: Some(info.export_format),
                        s3_bucket: info.s3_bucket,
                        s3_prefix: info.s3_prefix,
                        ..Default::default()
                    }),
                };
                wire::serialize_export_table_to_point_in_time_response(&output)
            }
            Err(e) => dynamodb_error_response(&e),
        }
    }

    async fn handle_describe_table_replica_auto_scaling(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_table_replica_auto_scaling_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        let table_name = input.table_name.clone();

        if let Err(e) = self
            .backend
            .describe_table(account_id, region, table_name.clone())
            .await
        {
            return dynamodb_error_response(&e);
        }

        let output = wire::DescribeTableReplicaAutoScalingOutput {
            table_auto_scaling_description: Some(wire::TableAutoScalingDescription {
                table_name: Some(table_name),
                table_status: Some("ACTIVE".to_string()),
                replicas: Some(vec![]),
            }),
        };
        wire::serialize_describe_table_replica_auto_scaling_response(&output)
    }

    async fn handle_update_table_replica_auto_scaling(
        &self,
        body: &[u8],
        account_id: String,
        region: String,
    ) -> MockResponse {
        let input = match wire::deserialize_update_table_replica_auto_scaling_request(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(
                    400,
                    "com.amazonaws.dynamodb.v20120810#ValidationException",
                    &e,
                );
            }
        };
        if input.table_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.dynamodb.v20120810#ValidationException",
                "Missing TableName",
            );
        }
        let table_name = input.table_name.clone();

        if let Err(e) = self
            .backend
            .describe_table(account_id, region, table_name.clone())
            .await
        {
            return dynamodb_error_response(&e);
        }

        let output = wire::UpdateTableReplicaAutoScalingOutput {
            table_auto_scaling_description: Some(wire::TableAutoScalingDescription {
                table_name: Some(table_name),
                table_status: Some("ACTIVE".to_string()),
                replicas: Some(vec![]),
            }),
        };
        wire::serialize_update_table_replica_auto_scaling_response(&output)
    }
}

/// Convert a Backup to a wire::BackupDescription.
fn backup_to_wire_description(backup: &Backup) -> wire::BackupDescription {
    let key_schema: Vec<wire::KeySchemaElement> = backup
        .key_schema
        .iter()
        .map(|k| wire::KeySchemaElement {
            attribute_name: k.attribute_name.clone(),
            key_type: k.key_type.clone(),
        })
        .collect();

    let provisioned = backup
        .provisioned_throughput
        .as_ref()
        .map(|pt| wire::ProvisionedThroughput {
            read_capacity_units: pt.read_capacity_units,
            write_capacity_units: pt.write_capacity_units,
        })
        .unwrap_or_default();

    wire::BackupDescription {
        backup_details: Some(wire::BackupDetails {
            backup_arn: Some(backup.backup_arn.clone()),
            backup_name: Some(backup.backup_name.clone()),
            backup_status: Some(backup.backup_status.clone()),
            backup_type: Some(backup.backup_type.clone()),
            backup_creation_date_time: Some(backup.backup_creation_date_time.timestamp() as f64),
            backup_size_bytes: Some(backup.backup_size_bytes),
            ..Default::default()
        }),
        source_table_details: Some(wire::SourceTableDetails {
            table_name: Some(backup.table_name.clone()),
            table_id: Some(backup.table_id.clone()),
            table_arn: Some(backup.table_arn.clone()),
            key_schema: Some(key_schema),
            table_creation_date_time: Some(backup.table_creation_date_time.timestamp() as f64),
            provisioned_throughput: Some(provisioned),
            item_count: Some(backup.item_count as i64),
            billing_mode: Some(backup.billing_mode.clone()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

// --- Conversion helpers ---

/// Convert a typed AttributeValue to a wire::AttributeValue.
fn attr_to_wire(av: &AttributeValue) -> wire::AttributeValue {
    match av {
        AttributeValue::S(s) => wire::AttributeValue {
            s: Some(s.clone()),
            ..Default::default()
        },
        AttributeValue::N(n) => wire::AttributeValue {
            n: Some(n.clone()),
            ..Default::default()
        },
        AttributeValue::B(b) => wire::AttributeValue {
            b: Some(b.clone()),
            ..Default::default()
        },
        AttributeValue::Bool(b) => wire::AttributeValue {
            b_o_o_l: Some(*b),
            ..Default::default()
        },
        AttributeValue::Null(n) => wire::AttributeValue {
            n_u_l_l: Some(*n),
            ..Default::default()
        },
        AttributeValue::SS(ss) => wire::AttributeValue {
            s_s: Some(ss.clone()),
            ..Default::default()
        },
        AttributeValue::NS(ns) => wire::AttributeValue {
            n_s: Some(ns.clone()),
            ..Default::default()
        },
        AttributeValue::BS(bs) => wire::AttributeValue {
            b_s: Some(bs.clone()),
            ..Default::default()
        },
        AttributeValue::L(l) => wire::AttributeValue {
            l: Some(l.iter().map(attr_to_wire).collect()),
            ..Default::default()
        },
        AttributeValue::M(m) => wire::AttributeValue {
            m: Some(
                m.iter()
                    .map(|(k, v)| (k.clone(), attr_to_wire(v)))
                    .collect(),
            ),
            ..Default::default()
        },
    }
}

/// Convert a serde_json::Value in DynamoDB JSON format to a wire::AttributeValue.
/// This is kept for use in code paths that receive raw JSON (e.g. ExecuteStatement results).
fn value_to_wire_attribute_value(value: &AttributeValue) -> wire::AttributeValue {
    attr_to_wire(value)
}

/// Convert a state Item (HashMap<String, AttributeValue>) to a wire item
/// (HashMap<String, wire::AttributeValue>).
fn convert_item_to_wire(item: &Item) -> HashMap<String, wire::AttributeValue> {
    item.iter()
        .map(|(k, v)| (k.clone(), attr_to_wire(v)))
        .collect()
}

/// Convert a Vec of state Items to a Vec of wire items.
fn convert_items_to_wire(items: &[Item]) -> Vec<HashMap<String, wire::AttributeValue>> {
    items.iter().map(convert_item_to_wire).collect()
}

/// Convert a types::Table to a wire::TableDescription.
fn table_to_wire_description(table: &Table) -> wire::TableDescription {
    let key_schema: Vec<wire::KeySchemaElement> = table
        .key_schema
        .iter()
        .map(|k| wire::KeySchemaElement {
            attribute_name: k.attribute_name.clone(),
            key_type: k.key_type.clone(),
        })
        .collect();

    let attr_defs: Vec<wire::AttributeDefinition> = table
        .attribute_definitions
        .iter()
        .map(|a| wire::AttributeDefinition {
            attribute_name: a.attribute_name.clone(),
            attribute_type: a.attribute_type.clone(),
        })
        .collect();

    let provisioned_throughput =
        table
            .provisioned_throughput
            .as_ref()
            .map(|pt| wire::ProvisionedThroughputDescription {
                read_capacity_units: Some(pt.read_capacity_units),
                write_capacity_units: Some(pt.write_capacity_units),
                number_of_decreases_today: Some(0),
                ..Default::default()
            });

    // FIX(terraform-e2e): For PAY_PER_REQUEST tables, AWS still returns
    // ProvisionedThroughput with zeros. The terraform provider expects this field.
    let provisioned_throughput =
        provisioned_throughput.or(Some(wire::ProvisionedThroughputDescription {
            read_capacity_units: Some(0),
            write_capacity_units: Some(0),
            number_of_decreases_today: Some(0),
            ..Default::default()
        }));

    let stream_specification = if table.stream_enabled || table.stream_view_type.is_some() {
        Some(wire::StreamSpecification {
            stream_enabled: table.stream_enabled,
            stream_view_type: table.stream_view_type.clone(),
        })
    } else {
        None
    };

    wire::TableDescription {
        table_name: Some(table.name.clone()),
        table_status: Some(table.table_status.clone()),
        table_arn: Some(table.arn.clone()),
        // FIX(terraform-e2e): TableId must be deterministic — random UUID per call
        // caused the terraform waiter to see a different response each time and poll
        // indefinitely. Derive a stable ID from the table ARN hash.
        table_id: Some(format!(
            "{:032x}",
            u128::from_be_bytes({
                use std::hash::{Hash, Hasher};
                let mut h = std::collections::hash_map::DefaultHasher::new();
                table.arn.hash(&mut h);
                let v = h.finish();
                let mut bytes = [0u8; 16];
                bytes[..8].copy_from_slice(&v.to_be_bytes());
                bytes[8..16].copy_from_slice(&v.wrapping_mul(0x9e3779b97f4a7c15).to_be_bytes());
                bytes
            })
        )),
        key_schema: Some(key_schema),
        attribute_definitions: Some(attr_defs),
        billing_mode_summary: Some(wire::BillingModeSummary {
            billing_mode: Some(table.billing_mode.clone()),
            ..Default::default()
        }),
        item_count: Some(table.item_count as i64),
        table_size_bytes: Some(0),
        creation_date_time: Some(table.creation_date_time.timestamp() as f64),
        provisioned_throughput,
        // FIX(terraform-e2e): Added — terraform reads this field on every DescribeTable.
        deletion_protection_enabled: Some(false),
        latest_stream_arn: table.latest_stream_arn.clone(),
        latest_stream_label: table.latest_stream_label.clone(),
        stream_specification,
        // FIX(terraform-e2e): newer terraform-provider-aws waiters require this field,
        // including WarmThroughput.Status — provider v6.13+ polls this field and treats
        // None/absent as '' which causes "unexpected state '', wanted target 'ACTIVE'".
        global_secondary_indexes: if table.global_secondary_indexes.is_empty() {
            None
        } else {
            Some(
                table
                    .global_secondary_indexes
                    .iter()
                    .map(|g| wire::GlobalSecondaryIndexDescription {
                        index_name: Some(g.index_name.clone()),
                        index_status: Some("ACTIVE".to_string()),
                        key_schema: Some(
                            g.key_schema
                                .iter()
                                .map(|k| wire::KeySchemaElement {
                                    attribute_name: k.attribute_name.clone(),
                                    key_type: k.key_type.clone(),
                                })
                                .collect(),
                        ),
                        projection: Some(wire::Projection {
                            projection_type: Some(g.projection_type.clone()),
                            non_key_attributes: if g.non_key_attributes.is_empty() {
                                None
                            } else {
                                Some(g.non_key_attributes.clone())
                            },
                        }),
                        index_arn: Some(format!("{}/index/{}", table.arn, g.index_name)),
                        item_count: Some(0),
                        index_size_bytes: Some(0),
                        ..Default::default()
                    })
                    .collect(),
            )
        },
        local_secondary_indexes: if table.local_secondary_indexes.is_empty() {
            None
        } else {
            Some(
                table
                    .local_secondary_indexes
                    .iter()
                    .map(|l| wire::LocalSecondaryIndexDescription {
                        index_name: Some(l.index_name.clone()),
                        key_schema: Some(
                            l.key_schema
                                .iter()
                                .map(|k| wire::KeySchemaElement {
                                    attribute_name: k.attribute_name.clone(),
                                    key_type: k.key_type.clone(),
                                })
                                .collect(),
                        ),
                        projection: Some(wire::Projection {
                            projection_type: Some(l.projection_type.clone()),
                            non_key_attributes: if l.non_key_attributes.is_empty() {
                                None
                            } else {
                                Some(l.non_key_attributes.clone())
                            },
                        }),
                        index_arn: Some(format!("{}/index/{}", table.arn, l.index_name)),
                        item_count: Some(0),
                        index_size_bytes: Some(0),
                    })
                    .collect(),
            )
        },
        warm_throughput: Some(wire::TableWarmThroughputDescription {
            read_units_per_second: Some(0),
            write_units_per_second: Some(0),
            status: Some(table.table_status.clone()),
        }),
        ..Default::default()
    }
}

// --- Utility functions ---

/// Build a Vec of state-side `KeySchemaElement` from a Vec of wire-side
/// `wire::KeySchemaElement` (the typed Smithy-derived input shape).
fn key_schema_from_wire(elements: &[wire::KeySchemaElement]) -> Vec<KeySchemaElement> {
    elements
        .iter()
        .map(|e| KeySchemaElement {
            attribute_name: e.attribute_name.clone(),
            key_type: e.key_type.clone(),
        })
        .collect()
}

/// Build a Vec of state-side `AttributeDefinition` from a Vec of wire-side
/// `wire::AttributeDefinition`.
fn attribute_definitions_from_wire(defs: &[wire::AttributeDefinition]) -> Vec<AttributeDefinition> {
    defs.iter()
        .map(|d| AttributeDefinition {
            attribute_name: d.attribute_name.clone(),
            attribute_type: d.attribute_type.clone(),
        })
        .collect()
}

/// Build state-side GSI definitions from the typed Smithy input vector.
fn gsi_defs_from_wire(indexes: &[wire::GlobalSecondaryIndex]) -> Vec<GlobalSecondaryIndexDef> {
    indexes
        .iter()
        .map(|idx| GlobalSecondaryIndexDef {
            index_name: idx.index_name.clone(),
            key_schema: key_schema_from_wire(&idx.key_schema),
            projection_type: idx
                .projection
                .projection_type
                .clone()
                .unwrap_or_else(|| "ALL".to_string()),
            non_key_attributes: idx
                .projection
                .non_key_attributes
                .clone()
                .unwrap_or_default(),
        })
        .collect()
}

/// Build state-side LSI definitions from the typed Smithy input vector.
fn lsi_defs_from_wire(indexes: &[wire::LocalSecondaryIndex]) -> Vec<LocalSecondaryIndexDef> {
    indexes
        .iter()
        .map(|idx| LocalSecondaryIndexDef {
            index_name: idx.index_name.clone(),
            key_schema: key_schema_from_wire(&idx.key_schema),
            projection_type: idx
                .projection
                .projection_type
                .clone()
                .unwrap_or_else(|| "ALL".to_string()),
            non_key_attributes: idx
                .projection
                .non_key_attributes
                .clone()
                .unwrap_or_default(),
        })
        .collect()
}

/// Walk an `Expr` AST produced from a `KeyConditionExpression`, collecting
/// equality predicates into `equalities` and the (at most one) range / function
/// condition into `sort_condition`.
fn walk_key_condition(
    expr: crate::expr::Expr,
    equalities: &mut Item,
    sort_condition: &mut Option<SortKeyCondition>,
) {
    use crate::expr::{CompOp, Expr, Operand};
    match expr {
        Expr::And(lhs, rhs) => {
            walk_key_condition(*lhs, equalities, sort_condition);
            walk_key_condition(*rhs, equalities, sort_condition);
        }
        Expr::Comparison(Operand::Path(name), op, Operand::Value(val)) => match op {
            CompOp::Eq => {
                equalities.insert(name, val);
            }
            CompOp::Lt => *sort_condition = Some(SortKeyCondition::LessThan(val)),
            CompOp::Le => *sort_condition = Some(SortKeyCondition::LessThanOrEqual(val)),
            CompOp::Gt => *sort_condition = Some(SortKeyCondition::GreaterThan(val)),
            CompOp::Ge => *sort_condition = Some(SortKeyCondition::GreaterThanOrEqual(val)),
            CompOp::Ne => {} // not valid in KeyConditionExpression
        },
        Expr::Between(Operand::Path(_), Operand::Value(lo), Operand::Value(hi)) => {
            *sort_condition = Some(SortKeyCondition::Between(lo, hi));
        }
        Expr::BeginsWith(_, Operand::Value(val)) => {
            *sort_condition = Some(SortKeyCondition::BeginsWith(val));
        }
        _ => {}
    }
}

fn global_table_to_wire(gt: &crate::types::GlobalTableInfo) -> wire::GlobalTableDescription {
    wire::GlobalTableDescription {
        global_table_name: Some(gt.global_table_name.clone()),
        global_table_arn: Some(gt.global_table_arn.clone()),
        global_table_status: Some(gt.global_table_status.clone()),
        creation_date_time: Some(gt.creation_date_time.timestamp() as f64),
        replication_group: Some(
            gt.replication_group
                .iter()
                .map(|r| wire::ReplicaDescription {
                    region_name: Some(r.clone()),
                    replica_status: Some("ACTIVE".to_string()),
                    ..Default::default()
                })
                .collect(),
        ),
    }
}

fn dynamodb_error_type(err: &DynamoDbError) -> &'static str {
    match err {
        DynamoDbError::ResourceInUse(_) => {
            "com.amazonaws.dynamodb.v20120810#ResourceInUseException"
        }
        DynamoDbError::NoHashKey => "com.amazonaws.dynamodb.v20120810#ValidationException",
        DynamoDbError::MissingKey(_) => "com.amazonaws.dynamodb.v20120810#ValidationException",
        DynamoDbError::QueryConditionMissedKey => {
            "com.amazonaws.dynamodb.v20120810#ValidationException"
        }
        DynamoDbError::ResourceNotFound(_) => {
            "com.amazonaws.dynamodb.v20120810#ResourceNotFoundException"
        }
        DynamoDbError::BackupNotFound(_) => {
            "com.amazonaws.dynamodb.v20120810#BackupNotFoundException"
        }
        DynamoDbError::TableAlreadyExists(_) => {
            "com.amazonaws.dynamodb.v20120810#TableAlreadyExistsException"
        }
        DynamoDbError::SourceTableNotFound(_) => {
            "com.amazonaws.dynamodb.v20120810#SourceTableNotFoundException"
        }
        DynamoDbError::GlobalTableAlreadyExists(_) => {
            "com.amazonaws.dynamodb.v20120810#GlobalTableAlreadyExistsException"
        }
        DynamoDbError::GlobalTableNotFound(_) => {
            "com.amazonaws.dynamodb.v20120810#GlobalTableNotFoundException"
        }
        DynamoDbError::KinesisDestinationNotFound { .. } => {
            "com.amazonaws.dynamodb.v20120810#ResourceNotFoundException"
        }
        DynamoDbError::TableNotFound(_) => {
            "com.amazonaws.dynamodb.v20120810#TableNotFoundException"
        }
        DynamoDbError::TableNotFoundByArn(_) => {
            "com.amazonaws.dynamodb.v20120810#TableNotFoundException"
        }
        DynamoDbError::StreamNotFound(_) => {
            "com.amazonaws.dynamodb.v20120810#ResourceNotFoundException"
        }
        DynamoDbError::ConditionalCheckFailed => {
            "com.amazonaws.dynamodb.v20120810#ConditionalCheckFailedException"
        }
        DynamoDbError::InternalError(_) => "com.amazonaws.dynamodb.v20120810#InternalError",
        DynamoDbError::ExportNotFound(_) => {
            "com.amazonaws.dynamodb.v20120810#ExportNotFoundException"
        }
        DynamoDbError::ImportNotFound(_) => {
            "com.amazonaws.dynamodb.v20120810#ImportNotFoundException"
        }
    }
}

fn dynamodb_error_response(err: &DynamoDbError) -> MockResponse {
    let (status, error_type) = match err {
        DynamoDbError::ResourceInUse(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#ResourceInUseException",
        ),
        DynamoDbError::NoHashKey => (400, "com.amazonaws.dynamodb.v20120810#ValidationException"),
        DynamoDbError::MissingKey(_) => {
            (400, "com.amazonaws.dynamodb.v20120810#ValidationException")
        }
        DynamoDbError::QueryConditionMissedKey => {
            (400, "com.amazonaws.dynamodb.v20120810#ValidationException")
        }
        DynamoDbError::ResourceNotFound(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#ResourceNotFoundException",
        ),
        DynamoDbError::BackupNotFound(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#BackupNotFoundException",
        ),
        DynamoDbError::TableAlreadyExists(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#TableAlreadyExistsException",
        ),
        DynamoDbError::SourceTableNotFound(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#SourceTableNotFoundException",
        ),
        DynamoDbError::GlobalTableAlreadyExists(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#GlobalTableAlreadyExistsException",
        ),
        DynamoDbError::GlobalTableNotFound(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#GlobalTableNotFoundException",
        ),
        DynamoDbError::KinesisDestinationNotFound { .. } => (
            400,
            "com.amazonaws.dynamodb.v20120810#ResourceNotFoundException",
        ),
        DynamoDbError::TableNotFound(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#TableNotFoundException",
        ),
        DynamoDbError::TableNotFoundByArn(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#TableNotFoundException",
        ),
        DynamoDbError::StreamNotFound(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#ResourceNotFoundException",
        ),
        DynamoDbError::ConditionalCheckFailed => (
            400,
            "com.amazonaws.dynamodb.v20120810#ConditionalCheckFailedException",
        ),
        DynamoDbError::InternalError(_) => (500, "com.amazonaws.dynamodb.v20120810#InternalError"),
        DynamoDbError::ExportNotFound(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#ExportNotFoundException",
        ),
        DynamoDbError::ImportNotFound(_) => (
            400,
            "com.amazonaws.dynamodb.v20120810#ImportNotFoundException",
        ),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

// ---------------------------------------------------------------------------
// Expression-evaluation helpers
// ---------------------------------------------------------------------------

/// Convert wire `AttributeValue` parameters used by PartiQL execute paths
/// into the `serde_json::Value` shape consumed by
/// `winterbaume_partiql::parse_statement`. Empty / absent input becomes an
/// empty vector, matching the previous behaviour.
fn wire_attrs_to_json(parameters: Option<Vec<crate::model::AttributeValue>>) -> Vec<Value> {
    parameters
        .unwrap_or_default()
        .into_iter()
        .map(|p| serde_json::to_value(p).unwrap_or(Value::Null))
        .collect()
}

/// Return a ConditionalCheckFailedException DynamoDbError.
fn conditional_check_failed() -> DynamoDbError {
    DynamoDbError::ConditionalCheckFailed
}

/// Look up and clone the item currently stored at `key` in `table_name`.
/// Returns an empty Item when the table or item does not exist.
fn get_existing_item(state: &crate::state::DynamoDbState, table_name: &str, key: &Item) -> Item {
    state
        .get_item(table_name, key)
        .ok()
        .flatten()
        .cloned()
        .unwrap_or_default()
}

/// Build a key Item for a PutItem by extracting the hash (and optional range)
/// key attributes from the item being written, using the table schema.
/// Returns None when the table doesn't exist (let put_item handle the error).
fn build_key_for_item(
    state: &crate::state::DynamoDbState,
    table_name: &str,
    item: &Item,
) -> Option<Item> {
    let ts = state.tables.get(table_name)?;
    let mut key = Item::new();
    if let Some(v) = item.get(&ts.table.hash_key_attr) {
        key.insert(ts.table.hash_key_attr.clone(), v.clone());
    }
    if let Some(ref rk) = ts.table.range_key_attr
        && let Some(v) = item.get(rk)
    {
        key.insert(rk.clone(), v.clone());
    }
    Some(key)
}

fn json_error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": error_type,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}

/// Extract the region from a DynamoDB request, handling both standard and
/// account-specific endpoint formats.
/// Standard: dynamodb.{region}.amazonaws.com
/// Account-specific: {account_id}.ddb.{region}.amazonaws.com
fn extract_dynamodb_region(request: &MockRequest) -> String {
    // Try to get region from the Authorization header's credential scope first
    if let Some(auth) = request
        .headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
    {
        // Parse: AWS4-HMAC-SHA256 Credential=AKID/20260224/us-east-1/dynamodb/aws4_request
        if let Some(cred_start) = auth.find("Credential=") {
            let cred = &auth[cred_start + 11..];
            let parts: Vec<&str> = cred.split('/').collect();
            if parts.len() >= 3 {
                return parts[2].to_string();
            }
        }
    }
    winterbaume_core::auth::extract_region_from_uri(&request.uri)
}

// (parse_index_defs / parse_lsi_defs removed; superseded by gsi_defs_from_wire
// and lsi_defs_from_wire that consume the codegen-typed input vectors.)
