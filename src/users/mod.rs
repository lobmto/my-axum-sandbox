use serde::Serialize;
use shaku::module;

pub mod controller;
pub mod service;

module! {
    pub AppModule {
        components = [service::ServiceImpl],
        providers = []
    }
}

#[derive(Serialize)]
pub struct Entity {
    pub id: u64,
    pub username: String,
}
