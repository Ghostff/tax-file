pub mod default_controller;
pub mod api_auth_controller;
pub mod user_controller;
pub mod tax_controller;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct OrganizationPathParams {
    pub id: Uuid,
}