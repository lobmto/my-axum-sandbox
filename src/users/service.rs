use axum::async_trait;
use shaku::{Component, Interface};

use crate::users;

#[async_trait]
pub trait Service: Interface {
    async fn create_user(&self, payload: CreateRequest) -> Result<users::Entity, local::Error>;
}

#[derive(Component)]
#[shaku(interface = Service)]
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
