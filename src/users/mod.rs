use axum::{routing::post, Router};
use serde::Serialize;

pub mod controller;
pub mod service;

pub fn routes() -> Router {
    Router::new().route("/", post(controller::create_user))
}

#[derive(Serialize)]
pub struct Entity {
    pub id: u64,
    pub username: String,
}
