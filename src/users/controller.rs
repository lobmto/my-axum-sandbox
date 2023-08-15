use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}