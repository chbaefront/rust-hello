// `pub mod`로 선언한 자식 모듈은 부모 모듈 밖에서도 사용할 수 있다.
// 실제 코드는 `network/client.rs`와 `network/server.rs`에 작성한다.
pub mod client;
pub mod server;

// `pub`이 없으므로 `protocol`은 network 모듈 내부에서만 사용할 수 있다.
// main.rs에서 `network::protocol`로 접근하면 비공개 모듈 오류가 발생한다.
mod protocol;

// 공개 함수이므로 `network::print_summary()` 경로로 호출할 수 있다.
pub fn print_summary() {
    // 비공개 자식 모듈도 부모인 network 모듈 안에서는 접근할 수 있다.
    println!(
        "네트워크 모듈: 기본 포트는 {}입니다.",
        protocol::DEFAULT_PORT
    );
}
