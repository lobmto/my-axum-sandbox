use super::service::{Service, ServiceImpl};
use crate::{users, users::service};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;

pub async fn create_user(
    Json(request): Json<CreateRequest>,
) -> Result<(StatusCode, Json<users::Entity>), service::Error> {
    let user = ServiceImpl {}.create_user(request.into()).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[derive(Deserialize)]
pub struct CreateRequest {
    pub username: String,
}
impl From<CreateRequest> for service::CreateRequest {
    fn from(value: CreateRequest) -> Self {
        service::CreateRequest {
            username: value.username,
        }
    }
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
