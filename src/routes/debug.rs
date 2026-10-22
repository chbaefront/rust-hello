use axum::{Router, routing::get, debug_handler};
use std::sync::{Arc, Mutex};

pub fn debug() -> Router<Arc<Mutex<Vec<i32>>>> {
    Router::new().route("/", get(get_handler))
}


#[debug_handler]
async fn get_handler() -> &'static str {
    "hello world!"
}
