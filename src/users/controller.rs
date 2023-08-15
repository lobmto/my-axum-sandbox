use super::{
    dto::{CreateUser, User},
    service::{Service, ServiceImpl},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), UnknownError> {
    let user = ServiceImpl {}.create_user(payload).await;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::CREATED, Json(user)))
}

pub struct UnknownError(anyhow::Error);
impl IntoResponse for UnknownError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}
