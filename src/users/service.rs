use std::sync::Arc;

use axum::async_trait;

use crate::users;

pub type DynService = Arc<dyn Service + Send + Sync>;

#[async_trait]
pub trait Service {
    async fn create_user(&self, payload: CreateRequest) -> Result<users::Entity, local::Error>;
}

pub struct ServiceImpl {}

#[async_trait]
impl Service for ServiceImpl {
    async fn create_user(&self, payload: CreateRequest) -> Result<users::Entity, local::Error> {
        Ok(users::Entity {
            id: 1234,
            username: payload.username,
        })
    }
}
pub struct CreateRequest {
    pub username: String,
}

pub use local::Error;
mod local {
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[allow(dead_code)]
        #[error("Unknown error: {0}")]
        UnknownError(String),
    }
}
