#[derive(Debug, Clone)]
pub struct SupportCase {
    pub case_id: String,
    pub display_id: String,
    pub subject: String,
    pub status: String,
    pub service_code: String,
    pub category_code: String,
    pub severity_code: String,
    pub communication_body: String,
    pub submitted_by: String,
    pub time_created: String,
    pub cc_email_addresses: Vec<String>,
    pub language: String,
}

#[derive(Debug, Clone)]
pub struct SupportService {
    pub code: String,
    pub name: String,
    pub categories: Vec<SupportCategory>,
}

#[derive(Debug, Clone)]
pub struct SupportCategory {
    pub code: String,
    pub name: String,
}
