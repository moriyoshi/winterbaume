use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ContactDetail {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub organization_name: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country_code: Option<String>,
    pub zip_code: Option<String>,
    pub contact_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DomainRegistration {
    pub domain_name: String,
    pub auto_renew: bool,
    pub admin_contact: ContactDetail,
    pub registrant_contact: ContactDetail,
    pub tech_contact: ContactDetail,
    pub admin_privacy: bool,
    pub registrant_privacy: bool,
    pub tech_privacy: bool,
    pub creation_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub transfer_lock: bool,
    pub status_list: Vec<String>,
    pub nameservers: Vec<String>,
}
