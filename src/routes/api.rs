use actix_web::web;
use actix_web::web::{ServiceConfig};
use crate::controllers::api::{api_auth_controller, default_controller, user_controller, tax_controller};
use crate::middlewares::auth_middleware::AuthMiddleware;

fn authenticated(cfg: &mut ServiceConfig) {
    cfg
        .route("/auth/me", get!(api_auth_controller::me))
        .route("/auth/profile", put!(api_auth_controller::update_profile))
        .route("/auth/delete-account", delete!(api_auth_controller::delete_account))
        .service(
            web::scope("/tax")
                .route("/documents", get!(tax_controller::list_documents))
                .route("/upload", post!(tax_controller::upload_document))
                .route("/data", post!(tax_controller::save_manual_data))
                .route("/data", get!(tax_controller::get_tax_data))
                .route("/ai-helper", post!(tax_controller::ai_helper))
                .route("/download/{id}", get!(tax_controller::download_document))
        )
        .service(
            web::scope("/users")
                .route("", get!(user_controller::index))
                .route("", post!(user_controller::create))
                .route("/{id}", get!(user_controller::show))
                .route("/{id}", put!(user_controller::update))
                .route("/{id}", delete!(user_controller::delete)),
        );
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.route("/health", get!(default_controller::health_check))
        .route("/auth/login", post!(api_auth_controller::login))
        .route("/auth/register", post!(api_auth_controller::register))
        .service(web::scope("").wrap(AuthMiddleware).configure(authenticated))
        .default_service(web::to(default_controller::page_not_found));
}
