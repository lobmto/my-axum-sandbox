use super::{
    dto::{CreateUser, User},
    service,
    service::{Service, ServiceImpl},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), service::Error> {
    let user = ServiceImpl {}.create_user(payload).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

impl IntoResponse for service::Error {
    fn into_response(self) -> Response {
        match self {
            service::Error::UnknownError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}
