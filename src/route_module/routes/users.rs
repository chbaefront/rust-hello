use axum::{routing::get, Router};

pub fn users_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }));
}