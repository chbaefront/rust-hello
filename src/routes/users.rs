use axum::{Router, routing::get};

pub fn users() -> Router {
    Router::new().route("/", get(|| async { "Hello, Users!!" }))
}
