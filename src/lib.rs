use std::sync::{Arc, Mutex};
use sea_orm::DatabaseConnection;

pub mod routes {
    pub mod users;
    pub mod state;
    pub mod debug;
}

pub mod entities;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub numbers: Arc<Mutex<Vec<i32>>>,
}