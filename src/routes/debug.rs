use axum::{Router, routing::get, debug_handler};
use crate::AppState;

pub fn debug() -> Router<AppState> {
    Router::new().route("/", get(get_handler))
}


#[debug_handler]
async fn get_handler() -> &'static str {
    "hello world!"
}
