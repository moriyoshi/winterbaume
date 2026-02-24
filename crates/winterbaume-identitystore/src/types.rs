use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Name {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honorific_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honorific_suffix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Email {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PhoneNumber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalId {
    pub issuer: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Role {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Photo {
    pub value: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

/// Stored group record in the identity store.
#[derive(Debug, Clone)]
pub struct Group {
    pub identity_store_id: String,
    pub group_id: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub external_ids: Vec<ExternalId>,
}

/// Stored group membership record in the identity store.
#[derive(Debug, Clone)]
pub struct GroupMembership {
    pub identity_store_id: String,
    pub membership_id: String,
    pub group_id: String,
    pub member_user_id: String,
}

/// Stored user record in the identity store.
#[derive(Debug, Clone)]
pub struct User {
    pub identity_store_id: String,
    pub user_id: String,
    pub user_name: Option<String>,
    pub name: Option<Name>,
    pub display_name: Option<String>,
    pub nick_name: Option<String>,
    pub profile_url: Option<String>,
    pub emails: Option<Vec<Email>>,
    pub addresses: Option<Vec<Address>>,
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    pub user_type: Option<String>,
    pub title: Option<String>,
    pub preferred_language: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub external_ids: Vec<ExternalId>,
    pub photos: Option<Vec<Photo>>,
    pub website: Option<String>,
    pub birthdate: Option<String>,
    pub roles: Option<Vec<Role>>,
}
