mod config;
mod services;
mod utilities;
mod routes;
mod models;
mod controllers;
mod middlewares;
mod repositories;

use std::io;
use std::sync::{LazyLock};
use std::time::{Duration};
use actix_cors::Cors;
use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_extensible_rate_limit::backend::SimpleInputFunctionBuilder;
use actix_extensible_rate_limit::RateLimiter;
use actix_web::middleware::NormalizePath;
use actix_web::{web, App, HttpServer};
use reqwest::Client;
use sqlx::postgres::{PgPoolOptions};
use sqlx::{Pool, Postgres};
use tracing::info;
use tracing_actix_web::TracingLogger;
use crate::config::ENV;
use crate::services::log_service;
use crate::utilities::error_bag::ErrorBag;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            // Set a realistic UserModel-Agent to prevent requests from being blocked by anti-bot protection.
            // This HTTP client is used throughout the application for:
            // - Web scraping to extract URL metadata (Open Graph tags, titles, descriptions)
            // Many services block requests without proper UserModel-Agent headers or flag generic ones as suspicious.
            let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36";
            match reqwest::header::HeaderValue::from_str(user_agent) {
                Ok(v) => { headers.insert(reqwest::header::USER_AGENT, v); }
                Err(e) => { eprintln!("ERROR: Invalid USER_AGENT header value: {:?}", e); }
            }
            headers
        })
        .build().unwrap_or_else(|e| {
            eprintln!("ERROR: Could not build HTTP client: {:?}", e);
            Client::new()
        })
});

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize tracing subscriber (reads RUST_LOG, e.g., "info", "debug", etc.)
    log_service::init_tracing();
    // Capture and log panics for better observability
    log_service::install_panic_hook();

    // Create PostgreSQL connection pool
    let pool = PgPoolOptions::new()
        .max_connections(64)
        .min_connections(4)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&ENV.database_url)
        .await
        .map_err(|e| {
            tracing::error!("Failed to connect to the database: {:?}", e);
            io::Error::new(io::ErrorKind::Other, "Database connection failed")
        })?;

    // Shared application state
    let state = AppState { pool: pool.clone() };


    // Custom JSON error handling to ensure
    // consistent error responses across the API
    let json_config = web::JsonConfig::default().error_handler(|err, _req| {
        ErrorBag::Deserialization(err.to_string()).into()
    });

    // Validator JSON configuration (for request validation)
    let validator_config = actix_web_validator::JsonConfig::default().error_handler(|err, _req| {
        ErrorBag::Deserialization(err.to_string()).into()
    });

    info!("Starting server at {}:{}", ENV.app_url, ENV.app_port);
    HttpServer::new(move || {
        // Throttle 60 req/sec
        // Limits incoming requests per IP
        let throttle = RateLimiter::builder(
            InMemoryBackend::builder().build(),
            SimpleInputFunctionBuilder::new(Duration::from_secs(1), 200).real_ip_key().build()
        ).add_headers().build();

        // API and Web CORS are separated
        // @todo: tightening
        let api_cors = Cors::permissive();
        let web_cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(json_config.clone())
            .app_data(validator_config.clone())
            .service(
                web::scope("/api")
                    .wrap(api_cors)
                    .wrap(throttle)
                    .configure(routes::api::init)
            )
            .service(web::scope("").wrap(web_cors).configure(routes::web::init))
            // Worker threads (usually = CPU cores)
            .wrap(TracingLogger::default())
            // Graceful shutdown timeout
            .wrap(NormalizePath::trim())
    })
    .bind(("0.0.0.0", ENV.app_port))?
    .workers(ENV.cpu_count)
    .shutdown_timeout(5)
    .run()
    .await
}
