use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Data;
use actix_web_validator::Json;
use chrono::Utc;
use serde_json::{json, to_value, Map, Value};
use crate::AppState;
use crate::models::user_model::{LoginUserSchema, UserModel, UpdateProfileSchema};
use crate::repositories::user_repository::UserRepository;
use crate::services::crypto_service::CryptoService;
use crate::services::jwt_service::JwtService;
use crate::services::user_service::UserService;
use crate::utilities::json_response::JsonResponse;
use crate::utilities::error_bag::ErrorBag;
use crate::utilities::http_request::HttpRequestExt;

const JWT_TTL_MINUTES: i64 = 60 * 24;

async fn build_user_response(user: &UserModel) -> Result<Map<String, Value>, ErrorBag> {
    let mut map = Map::new();

    map.insert("user".to_string(), to_value(user).map_err(|e| ErrorBag::InternalServerError(format!("Failed to serialize user: {}", e)))?);
    Ok(map)
}


pub async fn me(req: HttpRequest) -> Result<HttpResponse, ErrorBag> {
    let current_user = req.get_user();

    Ok(JsonResponse::success(json!(build_user_response(&current_user).await?)))
}

/// Auth handlers: keep HTTP concerns here, push crypto/DB/JWT to services/repos.
/// Guarantees:
/// - Validated JSON via `actix-web-validator::Json<T>`.
/// - No user-enumeration: auth failures use a single generic message.
/// - Consistent envelope via `JsonResponse` (success/error/fatal).
///
/// Security:
/// - Prevents user enumeration
/// - Avoids timing side-channel leaks
pub async fn login(body: Json<LoginUserSchema>, app: Data<AppState>) -> Result<HttpResponse, ErrorBag> {
    // Input is validated already; normalize email.
    let email = body.email.trim();

    // Lookup user; hide existence via generic error.
    let mut user = UserRepository::find_by_email(&app.pool, email).await
        .map_err(|e| match e {
            ErrorBag::NotFound(_) => ErrorBag::InvalidEmailOrPassword,
            _ => e
        })?;

    // Verify password; same generic error on mismatch.
    let crypto = CryptoService::new();
    if !crypto.verify_password(&user.password, &body.password) {
        return Err(ErrorBag::InvalidEmailOrPassword);
    }

    // Update login timestamps; ignore details in client response on failure.
    user.last_logged_in_at = user.current_logged_in_at.clone();
    user.current_logged_in_at = Some(Utc::now());
    UserRepository::update(&app.pool, &user).await?;

    // Issue 1-day JWT; prefix with Bearer for Authorization header use.
    let token = JwtService::create_access_token(user.id, JWT_TTL_MINUTES)
        .map(|t| format!("Bearer {t}"))
        .map_err(|e| ErrorBag::InternalServerError(format!("login.create_access_token failed: {:?}", e)))?;
    
    let mut response = build_user_response(&user).await?;
    response.insert("token".to_string(), to_value(token).unwrap_or_default());

    Ok(JsonResponse::success(json!(response)))
}

pub async fn update_profile(req: HttpRequest, body: Json<UpdateProfileSchema>, app: Data<AppState>) -> Result<HttpResponse, ErrorBag> {
    let mut current_user = (*req.get_user()).clone();
    let email = body.email.trim().to_lowercase();

    if current_user.email != email && UserRepository::email_exist(&app.pool, &email).await? {
        return Err(ErrorBag::EmailInUse);
    }

    current_user.first_name = body.first_name.clone();
    current_user.last_name = body.last_name.clone();
    current_user.email = email;

    if let Some(password) = &body.password {
        if !password.trim().is_empty() {
            let crypto = CryptoService::new();
            current_user.password = crypto.hash_password(password)
                .map_err(|e| ErrorBag::InternalServerError(format!("update_profile.hash_password failed: {:?}", e)))?;
        }
    }

    UserRepository::update(&app.pool, &current_user).await?;

    Ok(JsonResponse::success(json!({ "user": current_user, "message": "Profile updated successfully" })))
}

pub async fn delete_account(req: HttpRequest, app: Data<AppState>) -> Result<HttpResponse, ErrorBag> {
    let current_user = req.get_user();
    
    UserRepository::delete(&app.pool, &current_user.id).await?;

    Ok(JsonResponse::success(json!({ "message": "Account deleted successfully" })))
}

/// Registration flow:
/// - Ensures unique email (case-insensitive, pre-insert check).
/// - Hashes password securely using Argon2.
/// - Returns 201 Created + { user, token } on success.
///
/// Security:
/// - No password hash exposure.
/// - Email is normalized (trim + lowercase) before any DB call.
pub async fn register(body: Json<crate::models::user_model::CreateUserSchema>, app: Data<AppState>) -> Result<HttpResponse, ErrorBag> {
    let user = UserService::create(&app.pool, &body).await?;

    // Create access token (JWT)
    let token = JwtService::create_access_token(user.id, JWT_TTL_MINUTES)
        .map(|t| format!("Bearer {t}"))
        .map_err(|e| ErrorBag::InternalServerError(format!("register.create_access_token failed: {:?}", e)))?;

    let mut response = build_user_response(&user).await?;
    response.insert("token".to_string(), to_value(token).unwrap_or_default());

    Ok(JsonResponse::success(json!(response)))
}
