use crate::AppState;
use axum::{Router, debug_handler, routing::get};

pub fn debug() -> Router<AppState> {
    Router::new().route("/", get(get_handler))
}

#[debug_handler]
async fn get_handler() -> &'static str {
    "hello world!"
}
