use crate::users;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;

pub async fn create_user(
    State(service): State<users::service::DynService>,
    Json(request): Json<CreateRequest>,
) -> Result<(StatusCode, Json<users::Entity>), users::service::Error> {
    let user = service.create_user(request.into()).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[derive(Deserialize)]
pub struct CreateRequest {
    pub username: String,
}
impl From<CreateRequest> for users::service::CreateRequest {
    fn from(value: CreateRequest) -> Self {
        users::service::CreateRequest {
            username: value.username,
        }
    }
}

impl IntoResponse for users::service::Error {
    fn into_response(self) -> Response {
        match self {
            users::service::Error::UnknownError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}
