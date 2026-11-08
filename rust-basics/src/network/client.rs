// 형제 모듈에 접근하려면 부모를 뜻하는 `super`에서 경로를 시작한다.
use super::server;

// `pub` 함수이므로 main.rs에서 `network::client::send_data()`로 호출할 수 있다.
pub fn send_data(message: &str) {
    // server::address는 `pub(super)`이므로 부모인 network와 그 자식 모듈에서만 보인다.
    println!("{}에 데이터 전송: {message}", server::address());
}
