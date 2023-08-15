use super::{
    dto::{CreateUser, User},
    service::{Service, ServiceImpl},
};
use axum::{http::StatusCode, Json};

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = ServiceImpl {}.create_user(payload).await;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
