use axum::{
    routing::{get, },
    Router,
    response::IntoResponse
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    
    async fn callback() -> impl IntoResponse {
        "Hello, World!"
    }

    let app = Router::new().route("/", get(callback));
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
