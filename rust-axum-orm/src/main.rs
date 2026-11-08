use std::env;

use axum::{Router, routing::get};
use rust_axum_orm::{AppState, routes::users::users};
use sea_orm::{Database, DbErr};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    if let Err(error) = dotenvy::dotenv() {
        eprintln!(".env 파일을 로드하지 못했습니다: {error}");
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(database_url).await?;
    let state = AppState { db };

    let app = Router::new()
        .route("/", get(|| async { "Hello, SeaORM!" }))
        .nest("/users", users())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("rust-axum-orm server: http://localhost:3001");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
