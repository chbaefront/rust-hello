use sea_orm::DatabaseConnection;

pub mod entities;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
