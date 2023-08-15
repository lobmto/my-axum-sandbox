use axum::{routing::post, Router};

pub mod controller;
pub mod dto;

pub fn routes() -> Router {
    Router::new().route("/", post(controller::create_user))
}
