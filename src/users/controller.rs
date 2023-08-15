use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use shaku_axum::Inject;

use crate::users::{self, AppModule};

pub async fn create_user(
    service: Inject<AppModule, dyn users::service::Service>,
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
