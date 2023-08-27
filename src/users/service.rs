use std::sync::Arc;

use axum::async_trait;
use shaku::{Component, Interface};

use crate::users;

#[async_trait]
pub trait Service: Interface {
    async fn create_user(&self, payload: CreateRequest) -> Result<users::Entity, local::Error>;
}

#[derive(Component)]
#[shaku(interface = Service)]
pub struct ServiceImpl {
    #[shaku(inject)]
    repository: Arc<dyn users::repository::Repository>,
}

#[async_trait]
impl Service for ServiceImpl {
    async fn create_user(&self, payload: CreateRequest) -> Result<users::Entity, local::Error> {
        self.repository
            .create_user(payload.username)
            .await
            .map_err(Into::into)
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

impl From<users::repository::Error> for local::Error {
    fn from(value: users::repository::Error) -> local::Error {
        match value {
            users::repository::Error::UnknownError(_) => local::Error::UnknownError("".to_string()),
        }
    }
}
