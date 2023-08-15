use axum::async_trait;

use super::dto::{CreateUser, User};

#[async_trait]
pub trait Service {
    async fn create_user(&self, payload: CreateUser) -> Result<User, Error>;
}

pub struct ServiceImpl {}

#[async_trait]
impl Service for ServiceImpl {
    async fn create_user(&self, payload: CreateUser) -> Result<User, Error> {
        Ok(User {
            id: 1234,
            username: payload.username,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown error: {0}")]
    UnknownError(String),
}
