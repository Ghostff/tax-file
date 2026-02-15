use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::impl_model;
use crate::models::Model;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TaxDocumentModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub year: i32,
    pub document_type: String,
    pub file_name: String,
    pub file_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl_model!(TaxDocumentModel, "tax_documents", id);

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TaxDataModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub year: i32,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl_model!(TaxDataModel, "tax_data", id);

#[derive(Debug, Deserialize)]
pub struct SaveTaxDataSchema {
    pub year: i32,
    pub data: serde_json::Value,
}
