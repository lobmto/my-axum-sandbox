mod users;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};

use crate::users::AppModule;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let module = Arc::new(AppModule::builder().build());

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(users::controller::create_user))
        .with_state(AppState { module });

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Clone)]
struct AppState {
    module: Arc<AppModule>,
}

impl FromRef<AppState> for Arc<AppModule> {
    fn from_ref(app_state: &AppState) -> Arc<AppModule> {
        app_state.module.clone()
    }
}
