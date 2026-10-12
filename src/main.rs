use axum::{Router, response::IntoResponse, routing::get};

// mod route_module;
use rust_hello::routes::users::users;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // route_module::routes::test();

    async fn callback() -> impl IntoResponse {
        "Hello, World!"
    }

    let app = Router::new()
        .route("/", get(callback))
        .nest("/users", users());
        // .nest("/users", route_module::routes::users::users_router());
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
