#[derive(Debug, Clone)]
pub struct Budget {
    pub budget_name: String,
    pub budget_type: String,
    pub budget_limit_amount: String,
    pub budget_limit_unit: String,
    pub time_unit: String,
    pub notifications: Vec<Notification>,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub notification_type: String,
    pub comparison_operator: String,
    pub threshold: f64,
    pub threshold_type: String,
}
