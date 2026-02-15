//! Authentication extractors for Actix handlers.
//!
//! This module provides two request extractors:
//! - `UserModel`: requires a valid Authorization: Bearer <JWT> header. If missing/invalid, the request
//!   is rejected with Unauthorized.
//! - `OptionalUser`: does the same lookup but never fails the extraction. It yields
//!   `OptionalUser(Some(UserModel))` when a valid token is present and maps any missing/invalid token to
//!   `OptionalUser(None)`. This is useful for endpoints where authentication is optional, e.g.
//!   personalized-but-public pages.
//!
//! Quick examples
//!
//! - Required login:
//!   ```ignore
//!   use actix_web::{get, HttpResponse};
//!   use crate::models::user_model::UserModel;
//!
//!   #[get("/me")]
//!   async fn me(req: HttpRequest) -> HttpResponse {
//!       let user = req.get_user();
//!       HttpResponse::Ok().json(user)
//!   }
//!   ```
//!
//! - Optional login:
//!   ```ignore
//!   use actix_web::{get, HttpResponse};
//!   use crate::models::user_model::OptionalUser;
//!
//!   #[get("/home")]
//!   async fn home(user: OptionalUser) -> HttpResponse {
//!       if let Some(user) = user {
//!           HttpResponse::Ok().body(format!("Welcome back, {}!", user.first_name))
//!       } else {
//!           HttpResponse::Ok().body("Welcome, guest!")
//!       }
//!   }
//!   ```
//!
//! Notes for future maintainers and AI assistants:
//! - Token parsing is lenient: missing header, invalid UTF-8, wrong prefix, bad signature, or a user
//!   not found all translate to `OptionalUser(None)` but to Unauthorized for `UserModel`.
//! - We depend on `AppState` being registered with actix-web's app data and `JwtService` returning
//!   a `sub` claim that can be parsed into a UUID.
//! - Keep extractor behavior side-effect free: do not log errors or cause delays here; let handlers
//!   decide on responses.

use std::future::{ready, Ready};
use std::rc::Rc;
use std::sync::Arc;
use crate::models::user_model::{UserModel};
use crate::repositories::user_repository::UserRepository;
use crate::services::jwt_service::JwtService;
use crate::utilities::error_bag::ErrorBag;
use crate::{AppState};
use actix_web::{dev::Payload, web, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::web::Data;
use futures::future::{LocalBoxFuture};
use uuid::Uuid;
use crate::utilities::str::FilterEmptyString;

/// Attempts to resolve a `UserModel` from the incoming request by:
/// 1) Reading the `Authorization` header and extracting the Bearer token
/// 2) Verifying the token via `JwtService`
/// 3) Parsing the subject claim (`sub`) as a UUID
/// 4) Loading the user from the database
///
/// Returns `None` for any failure along the way.
async fn fetch_user_from_request(req: &HttpRequest) -> Option<UserModel> {
    let token = match req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .empty_as_none()
        .unwrap_or_default()
        .strip_prefix("Bearer ")
    {
        Some(token) => token,
        None => return None,
    };

    let app = req.app_data::<web::Data<AppState>>()?.get_ref().clone();
    // Validate JWT and extract user ID
    let user_id = match JwtService::verify_access_token(token) {
        Ok(claims) => match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(_) => return None,
        },
        Err(_) => return None,
    };

    // Lookup user
    if let Ok(user) = UserRepository::find_by_id(&app.pool, &user_id).await {
        return Some(user);
    }

    None
}


/// Strict authentication extractor.
///
/// Fails with `ErrorBag::Unauthorized` when the request does not contain a valid Bearer token
/// and corresponding user record. Succeeds and yields the `UserModel` otherwise.
impl FromRequest for UserModel {
    type Error = ErrorBag;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            match fetch_user_from_request(&req).await {
                Some(user) => Ok(user),
                None => Err(ErrorBag::Unauthorized),
            }
        })
    }
}

// There are two steps in middleware processing.
// 1. Middleware initialization: the middleware factory is created and
//    receives the next service in the chain.
// 2. Middleware execution: the `call` method is invoked per request.
pub struct AuthMiddleware;

// Middleware factory implements the `Transform` trait
// `S` - the next service in the middleware chain
impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service: Rc::new(service), }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        // Extract Bearer token from Authorization header
        let token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(str::to_owned);

        Box::pin(async move {
            // Helper to reject unauthorized requests
            let reject = |r: ServiceRequest| {
                Ok(r.into_response(HttpResponse::Unauthorized().finish()).map_into_boxed_body())
            };

            // Helper to continue request processing with authenticated user
            let proceed = async |r: ServiceRequest, u: UserModel| {
                // Attach authenticated user to request extensions
                r.extensions_mut().insert(Arc::new(u));
                srv.call(r).await
            };

            // Access application state
            let app = match req.app_data::<Data<AppState>>() {
                Some(a) => a,
                None => return reject(req),
            };

            let token = match token {
                Some(t) => t,
                None => return reject(req),
            };

            // Validate JWT and extract claims
            let claims = match JwtService::verify_access_token(&token) {
                Ok(c) => c,
                Err(_) => return reject(req),
            };

            // Convert JWT subject into UUID
            let user_id = match Uuid::parse_str(&claims.sub) {
                Ok(id) => id,
                Err(_) => return reject(req),
            };

            // Cache miss: fetch user from database
            match UserRepository::find_by_id(&app.pool, &user_id).await {
                Ok(u) => proceed(req, u).await,
                Err(_) => return reject(req),
            }
        })
    }
}

