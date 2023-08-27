use axum::async_trait;
use shaku::{Component, Interface};

use crate::users;

#[async_trait]
pub trait Repository: Interface {
    async fn create_user(&self, username: String) -> Result<users::Entity, local::Error>;
}

#[derive(Component)]
#[shaku(interface = Repository)]
pub struct RepositoryImpl {}

#[async_trait]
impl Repository for RepositoryImpl {
    async fn create_user(&self, username: String) -> Result<users::Entity, local::Error> {
        Ok(users::Entity { id: 1234, username })
    }
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
