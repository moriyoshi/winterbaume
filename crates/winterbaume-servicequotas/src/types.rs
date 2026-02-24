#[derive(Debug, Clone)]
pub struct ServiceQuotaEntry {
    pub service_code: String,
    pub service_name: String,
    pub quota_code: String,
    pub quota_name: String,
    pub quota_arn: String,
    pub value: f64,
    pub unit: String,
    pub adjustable: bool,
    pub global_quota: bool,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ServiceEntry {
    pub service_code: String,
    pub service_name: String,
}
