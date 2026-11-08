use axum::http::{HeaderValue, Method, header};
use axum::{Router, response::IntoResponse, routing::get};
use rust_axum::AppState;
use rust_axum::routes::{debug::debug, state::state, users::users};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .allow_credentials(true);

    async fn callback() -> impl IntoResponse {
        "Hello, World!"
    }
    let app_state = AppState {
        numbers: Arc::new(Mutex::new(vec![0, 2, 3])),
    };

    let app = Router::new()
        .route("/", get(callback))
        .nest("/users", users())
        .nest("/state", state())
        .nest("/debug", debug())
        .layer(cors)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("rust-axum server: http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
