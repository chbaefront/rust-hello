use std::sync::{Arc, Mutex};
use axum::{Router, response::IntoResponse, routing::get};
use axum::http::{header, Method};
// mod route_module;
use rust_hello::routes::users::users;
use rust_hello::routes::state::state;
use rust_hello::routes::debug::debug;

// cors
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // route_module::routes::test();

    let cors = CorsLayer::new()
        .allow_origin(Any) // 모든 도메인 허용 = Any
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    async fn callback() -> impl IntoResponse {
        "Hello, World!"
    }

    // 공유 되는 상태
    // let data = vec![0; 3];
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let app = Router::new()
        .route("/", get(callback))
        .nest("/users", users())
        .nest("/state", state())
        .nest("/debug", debug())
        .layer(cors)
        .with_state(data);
    // .nest("/users", route_module::routes::users::users_router());
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
