use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::impl_model;
use crate::models::Model;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AddressModel {
    pub id: Uuid,
    pub address_raw: String,
    pub street_address: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub route_number: Option<String>,
    pub route_prefix: Option<String>,
    pub route: Option<String>,
    pub route_type: Option<String>,
    pub route_suffix: Option<String>,
    pub unit_type: Option<String>,
    pub unit_number: Option<String>,
    pub intersection: Option<String>,
    pub country_id: Uuid,
    pub state_id: Option<Uuid>,
    pub county: Option<String>,
    pub locality: Option<String>,
    pub sublocality: Option<String>,
    pub subdivision: Option<String>,
    pub neighborhood: Option<String>,
    pub school_district: Option<String>,
    pub zip: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl_model!(AddressModel, "addresses", id);
