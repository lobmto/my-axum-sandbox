use axum::async_trait;

use super::dto::{CreateUser, User};

#[async_trait]
pub trait Service {
    async fn create_user(&self, payload: CreateUser) -> User;
}

pub struct ServiceImpl {}

#[async_trait]
impl Service for ServiceImpl {
    async fn create_user(&self, payload: CreateUser) -> User {
        User {
            id: 1234,
            username: payload.username,
        }
    }
}
