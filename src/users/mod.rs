use std::sync::Arc;

use axum::{routing::post, Router};
use serde::Serialize;

pub mod controller;
pub mod service;

pub fn routes() -> Router {
    let user_service = Arc::new(service::ServiceImpl {}) as service::DynService;

    Router::new()
        .route("/", post(controller::create_user))
        .with_state(user_service)
}

#[derive(Serialize)]
pub struct Entity {
    pub id: u64,
    pub username: String,
}
