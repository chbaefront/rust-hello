use super::protocol;

// 외부에 공개한 서버 연결 함수다.
pub fn connect() {
    println!("{}에 연결했습니다.", address());
}

// `pub(super)`는 이 함수를 부모인 network 모듈 범위까지만 공개한다.
// 따라서 형제인 client 모듈에서는 사용할 수 있지만 main.rs에서는 직접 호출할 수 없다.
pub(super) fn address() -> String {
    format!("127.0.0.1:{}", protocol::DEFAULT_PORT)
}
