use actix_web::{web, HttpMessage, HttpRequest};
use std::collections::HashMap;
use std::sync::Arc;
use crate::models::user_model::UserModel;

pub trait HttpRequestExt {
    fn get_query(&self, name: &str) -> Option<String>;
    fn get_user(&self) -> Arc<UserModel>;
}

impl HttpRequestExt for HttpRequest {
    /// Get a value from the query string by name
    fn get_query(&self, name: &str) -> Option<String> {
        web::Query::<HashMap<String, String>>::from_query(self.query_string())
            .ok()
            .and_then(|query| query.get(name).cloned())
    }

    fn get_user(&self) -> Arc<UserModel> {
        self.extensions()
            .get::<Arc<UserModel>>()
            .cloned()
            .unwrap_or_else(|| {
                panic!("HttpRequest::get_user called on a route without AuthMiddleware or where authentication failed. Check your route configuration.");
            })
    }
}
