use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn users() -> Router {
    Router::new().route("/", get(|| async { "Hello, Users!" }))
}
