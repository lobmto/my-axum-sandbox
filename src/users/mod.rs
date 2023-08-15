use axum::{routing::post, Router};

pub mod controller;

pub fn routes() -> Router {
    Router::new().route("/", post(controller::create_user))
}
