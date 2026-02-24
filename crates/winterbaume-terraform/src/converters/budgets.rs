//! Terraform converter for Budgets resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_budgets::BudgetsService;
use winterbaume_budgets::views::{BudgetView, BudgetsStateView, NotificationView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_budgets_budget
// ---------------------------------------------------------------------------

/// Converts `aws_budgets_budget` Terraform resources to/from Budgets state.
pub struct AwsBudgetsBudgetConverter {
    service: Arc<BudgetsService>,
}

impl AwsBudgetsBudgetConverter {
    pub fn new(service: Arc<BudgetsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBudgetsBudgetConverter {
    fn resource_type(&self) -> &str {
        "aws_budgets_budget"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsBudgetsBudgetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let budget_name = require_str(attrs, "name", "aws_budgets_budget")?;
        let budget_type = optional_str(attrs, "budget_type").unwrap_or_else(|| "COST".to_string());
        let region = extract_region(attrs, &ctx.default_region);

        let budget_limit_amount =
            optional_str(attrs, "limit_amount").unwrap_or_else(|| "0".to_string());
        let budget_limit_unit =
            optional_str(attrs, "limit_unit").unwrap_or_else(|| "USD".to_string());
        let time_unit = optional_str(attrs, "time_unit").unwrap_or_else(|| "MONTHLY".to_string());
        let _tags_all = attrs.get("tags_all");
        let _time_period_start = optional_str(attrs, "time_period_start");
        let _time_period_end = optional_str(attrs, "time_period_end");
        let _cost_filter = attrs.get("cost_filter");
        let auto_adjust_data = attrs.get("auto_adjust_data").cloned();
        let cost_types = attrs.get("cost_types").cloned();
        let planned_limit = attrs.get("planned_limit").cloned();

        // Parse notifications from Terraform nested blocks
        let notifications = if let Some(notif_arr) =
            attrs.get("notification").and_then(|v| v.as_array())
        {
            notif_arr
                .iter()
                .filter_map(|n| {
                    let notification_type = n.get("notification_type")?.as_str()?.to_string();
                    let comparison_operator = n.get("comparison_operator")?.as_str()?.to_string();
                    let threshold = n.get("threshold")?.as_f64()?;
                    let threshold_type = n
                        .get("threshold_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("PERCENTAGE")
                        .to_string();
                    Some(NotificationView {
                        notification_type,
                        comparison_operator,
                        threshold,
                        threshold_type,
                    })
                })
                .collect()
        } else {
            vec![]
        };

        let budget_view = BudgetView {
            budget_name: budget_name.to_string(),
            budget_type,
            budget_limit_amount,
            budget_limit_unit,
            time_unit,
            notifications,
            auto_adjust_data,
            cost_types,
            planned_limit,
        };

        let mut state_view = BudgetsStateView {
            budgets: HashMap::new(),
            ..Default::default()
        };
        state_view
            .budgets
            .insert(budget_name.to_string(), budget_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for budget in view.budgets.values() {
            let notifications_json: Vec<serde_json::Value> = budget
                .notifications
                .iter()
                .map(|n| {
                    serde_json::json!({
                        "notification_type": n.notification_type,
                        "comparison_operator": n.comparison_operator,
                        "threshold": n.threshold,
                        "threshold_type": n.threshold_type,
                    })
                })
                .collect();
            let arn = format!(
                "arn:aws:budgets::{}:budget/{}",
                ctx.default_account_id, budget.budget_name
            );
            let attrs = serde_json::json!({
                "id": budget.budget_name,
                "name": budget.budget_name,
                "budget_type": budget.budget_type,
                "limit_amount": budget.budget_limit_amount,
                "limit_unit": budget.budget_limit_unit,
                "time_unit": budget.time_unit,
                "notification": notifications_json,
                "tags_all": {},
                "cost_filter": [],
                "arn": arn,
                "auto_adjust_data": budget.auto_adjust_data,
                "cost_types": budget.cost_types,
                "planned_limit": budget.planned_limit,
            });
            results.push(ExtractedResource {
                name: budget.budget_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
