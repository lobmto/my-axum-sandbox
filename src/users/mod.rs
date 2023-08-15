use axum::{routing::post, Router};

pub mod controller;
pub mod dto;
pub mod service;

pub fn routes() -> Router {
    Router::new().route("/", post(controller::create_user))
}
