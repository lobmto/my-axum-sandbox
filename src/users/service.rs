use axum::async_trait;

use super::dto::CreateUser;
use crate::users;

#[async_trait]
pub trait Service {
    async fn create_user(&self, payload: CreateUser) -> Result<users::Entity, service::Error>;
}

pub struct ServiceImpl {}

#[async_trait]
impl Service for ServiceImpl {
    async fn create_user(&self, payload: CreateUser) -> Result<users::Entity, service::Error> {
        Ok(users::Entity {
            id: 1234,
            username: payload.username,
        })
    }
}

pub use service::Error;
mod service {
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Unknown error: {0}")]
        UnknownError(String),
    }
}
