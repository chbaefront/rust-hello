use crate::AppState;
use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;

pub fn state() -> Router<AppState> {
    Router::new().route("/", get(state_hello))
}

async fn state_hello(State(data): State<AppState>) -> impl IntoResponse {
    // data[0] += 1;
    let mut target_data = data.numbers.lock().unwrap(); // 데이터에 접근하려면 명시적으로 lock을 해줘야함.
    target_data[0] += 1;
    println!("{:?}", target_data);

    format!("Hello, world! {}!", target_data[0])
}
