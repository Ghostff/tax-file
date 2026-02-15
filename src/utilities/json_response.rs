use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

/// JsonResponse centralizes common JSON HTTP responses for controllers.
///
/// Why this exists (AI-friendly summary):
/// - Keeps controllers thin and consistent across the codebase.
/// - Provides a single place to tweak response envelope (status/message/data) and logging.
/// - Each helper returns an `HttpResponse` ready to be returned from handlers.
///
/// Usage examples:
/// - Return a success with custom root fields: `JsonResponse::success_with(json!({ "user": user }))`
/// - Return a 400 error: `JsonResponse::error("Invalid input")`
/// - Return a 401 error: `JsonResponse::unauthorized("Unauthorized")`
/// - Log and return 500: `JsonResponse::fatal(err, "login query failed")`
pub struct JsonResponse;

impl JsonResponse {
    /// Convenience success that takes any serializable payload and nests it under `data`.
    pub fn success<T: Serialize>(data: T) -> HttpResponse {
        HttpResponse::Ok().json(json!({"status": "success","data": data}))
    }
}