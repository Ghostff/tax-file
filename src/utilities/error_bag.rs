use actix_web::http::StatusCode;
use std::fmt;
use actix_web::HttpResponse;
use serde_json::{Map, Value};
use tracing::error;

/// ErrorBag: centralized, stable, AI-friendly application errors.
///
/// What it is:
/// - A small enum of known, client-facing error cases used across handlers/services.
/// - Each variant carries:
///   - status_code(): HTTP status to return
///   - error_code(): stable machine code (enum variant name)
///   - message(): short human-readable text
/// - Converts itself to a JSON body via to_json() and to an HttpResponse via Responder/ResponseError.
///
/// Why it exists:
/// - Single source of truth for user-facing errors (prevents ad-hoc strings/status codes).
/// - Stable automation surface: tools/clients/LLMs can reliably key off error_code.
/// - Safer API design: avoids leaking internals while keeping messages actionable.
///
/// AI usage guidelines:
/// - Prefer existing variants over inventing new ad-hoc strings.
/// - Choose the least-revealing variant for auth (e.g., InvalidEmailOrPassword).
/// - For field errors, use Validation { field, message } with concise, neutral text.
/// - For missing entities, use NotFound { entity } with a singular, lowercase entity name (e.g., "user").
/// - If you need a new business error:
///   1) Add a new enum variant
///   2) Map its StatusCode in status_code()
///   3) Provide a succinct message() string
///   4) Keep error_code() stable (do not rename variants casually)
///
/// JSON shape produced by to_json():
/// {
///   "error": { "message": "<human message>" },
///   "code":  "<stable_error_code>"
/// }
///
/// Interop:
/// - Handlers can return ErrorBag directly (Responder) or wrap via helpers that return HttpResponse.
/// - Downstream clients/agents should branch on "code", not on the "message".
#[derive(Debug, Clone)]
pub enum ErrorBag {
    /// Generic invalid credentials for login flows.
    /// Use this instead of revealing whether the email or password was incorrect.
    InvalidEmailOrPassword,
    Unauthorized,

    EmailInUse,
    InternalServerError(String),
    BadRequest(String),

    /// Entity-not-found style error with the entity name.
    NotFound(String),

    /// Validation error for a specific field with a custom message.
    Validation { field: String, message: String },

    /// Generic deserialization error
    Deserialization(String),
    IntegrationError(String),

    Forbidden,
    Conflict(String),

    /// Database-related errors
    Database(String),

    /// HTTP client errors (e.g., from reqwest)
    Http(String),

    /// JSON parsing errors
    Json(String),

    /// External service/API errors
    ExternalService { service: String, message: String },
}

impl ErrorBag {
    /// Map each error to an appropriate HTTP status code.
    pub fn status_code(&self) -> StatusCode {
        match self {
            ErrorBag::InvalidEmailOrPassword => StatusCode::BAD_REQUEST,
            ErrorBag::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorBag::EmailInUse => StatusCode::CONFLICT,
            ErrorBag::NotFound(_) => StatusCode::NOT_FOUND,
            ErrorBag::Validation { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorBag::Deserialization(_) => StatusCode::BAD_REQUEST,
            ErrorBag::Forbidden => StatusCode::FORBIDDEN,
            ErrorBag::Conflict(_) => StatusCode::CONFLICT,
            ErrorBag::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorBag::IntegrationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorBag::BadRequest(_) => StatusCode::BAD_REQUEST,
            ErrorBag::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorBag::Http(_) => StatusCode::BAD_GATEWAY,
            ErrorBag::Json(_) => StatusCode::BAD_REQUEST,
            ErrorBag::ExternalService { .. } => StatusCode::BAD_GATEWAY,
        }
    }

    /// Human-friendly message. Keep messages short and neutral.
    pub fn message(&self) -> String {
        match self {
            ErrorBag::InvalidEmailOrPassword => "Invalid email or password".to_string(),
            ErrorBag::Unauthorized => "Unauthorized".to_string(),
            ErrorBag::EmailInUse => "It looks like this email address is already registered.".to_string(),
            ErrorBag::NotFound(entity) => format!("{entity} not found"),
            ErrorBag::Validation { field, message } => format!("{field}: {message}"),
            ErrorBag::Deserialization(msg) => msg.clone(),
            ErrorBag::Forbidden => "You do not have permission to perform this action.".to_string(),
            ErrorBag::Conflict(msg) => msg.clone(),
            ErrorBag::BadRequest(msg) => msg.clone(),
            ErrorBag::InternalServerError(msg) => msg.clone(),
            ErrorBag::IntegrationError(msg) => msg.clone(),
            ErrorBag::Database(msg) => format!("Database error: {msg}"),
            ErrorBag::Http(msg) => format!("HTTP error: {msg}"),
            ErrorBag::Json(msg) => format!("JSON error: {msg}"),
            ErrorBag::ExternalService { message, .. } => format!("{message}"),
        }
    }

    /// Stable error code equal to the enum variant name (e.g., "InvalidEmailOrPassword").
    pub fn error_code(&self) -> String {
        let dbg = format!("{:?}", self);
        let end = dbg.find(['(', ' ']).unwrap_or(dbg.len());

        dbg[..end].to_string()
    }

    pub fn to_json(&self) -> Value {
        let mut map = Map::new();
        let mut error_map = Map::new();
        error_map.insert("message".to_string(), Value::String(self.message()));

        map.insert("status".to_string(), Value::String("error".to_string()));
        map.insert("error".to_string(), Value::Object(error_map));
        map.insert("code".to_string(), Value::String(self.error_code()));

        map.into()
    }
}

impl fmt::Display for ErrorBag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl actix_web::ResponseError for ErrorBag {
    fn status_code(&self) -> StatusCode {
        // Call the inherent method via fully qualified syntax to avoid recursion
        ErrorBag::status_code(self)
    }

    fn error_response(&self) -> HttpResponse {
        // Log only NON-USER errors (i.e., server-side / 5xx classes)
        let status = ErrorBag::status_code(self);
        if status.is_server_error() {
            match self {
                ErrorBag::ExternalService { service, .. } => {
                    error!(
                        error_code = %self.error_code(),
                        http_status = %status.as_u16(),
                        service = %service,
                        ?self,
                        "non-user error"
                    );
                }
                _ => {
                    error!(
                        error_code = %self.error_code(),
                        http_status = %status.as_u16(),
                        ?self,
                        "non-user error"
                    );
                }
            }
        }

        // Build a sanitized JSON response for the client
        HttpResponse::build(status).json(self.to_json())
    }
}

impl actix_web::Responder for ErrorBag {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        // Mirror ResponseError behavior and log only non-user errors when returned directly
        let status = ErrorBag::status_code(&self);
        if status.is_server_error() {
            match &self {
                ErrorBag::ExternalService { service, .. } => {
                    error!(
                        error_code = %self.error_code(),
                        http_status = %status.as_u16(),
                        service = %service,
                        ?self,
                        "non-user error"
                    );
                }
                _ => {
                    error!(
                        error_code = %self.error_code(),
                        http_status = %status.as_u16(),
                        ?self,
                        "non-user error"
                    );
                }
            }
        }

        HttpResponse::build(status).json(self.to_json())
    }
}

impl From<sqlx::Error> for ErrorBag {
    fn from(err: sqlx::Error) -> Self {
        let error_string = err.to_string().to_lowercase();

        if error_string.contains("duplicate") || error_string.contains("unique constraint") {
            // Unique name per owner/system
            if error_string.contains("uniq_integrations_name_owner_system") {
                return ErrorBag::Conflict("An integration with this name already exists for this owner".into());
            }
            // Only one default per owner/system
            if error_string.contains("uniq_integrations_default_per_owner_system") {
                return ErrorBag::Conflict("Only one default integration is allowed per owner and system. The other integrations will be unset automatically when you set this as default. Please retry.".into());
            }
            // Legacy/renamed index guard (kept for backward compatibility in logs)
            if error_string.contains("uniq_integrations_owner_system") {
                return ErrorBag::Conflict("An integration of this type already exists for this owner (check if default)".into());
            }
            return ErrorBag::Conflict("An integration with this name or configuration already exists".into());
        }

        if error_string.contains("default_must_exist") {
            return ErrorBag::Validation {
                field: "is_default".to_string(),
                message: "A default integration must exist. Please set this integration as default or ensure another default exists.".into(),
            };
        }

        if error_string.contains("decode") || error_string.contains("deserialize") || error_string.contains("json") {
            return ErrorBag::Validation {
                field: "data".to_string(),
                message: format!("Invalid integration data: {}", err),
            };
        }

        if error_string.contains("organizations_slug_key") || error_string.contains("idx_org_slug") {
            return ErrorBag::Conflict("An organization with this slug already exists".into());
        }
        if error_string.contains("uq_org_parent_name") {
            return ErrorBag::Conflict("An organization with this name already exists under this parent".into());
        }

        match err {
            sqlx::Error::RowNotFound => ErrorBag::NotFound("Record".into()),
            _ => ErrorBag::Database(err.to_string()),
        }
    }
}

impl From<reqwest::Error> for ErrorBag {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            return ErrorBag::Http("Request timed out".into());
        }

        if err.is_connect() {
            return ErrorBag::Http("Connection failed".into());
        }

        if let Some(status) = err.status() {
            return ErrorBag::Http(format!("HTTP {}: {}", status, err));
        }

        ErrorBag::Http(err.to_string())
    }
}

impl From<serde_json::Error> for ErrorBag {
    fn from(err: serde_json::Error) -> Self {
        ErrorBag::Json(err.to_string())
    }
}
