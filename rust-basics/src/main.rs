// `mod network;`는 컴파일러에게 `network.rs` 또는 `network/mod.rs`에서
// `network` 모듈의 내용을 찾으라고 알려준다.
mod network;

// `storage` 모듈은 `storage.rs` 대신 `storage/mod.rs`를 루트 파일로 사용한다.
// 이 방식은 이전 Rust 프로젝트에서 널리 사용됐으며 현재도 정상적으로 지원된다.
mod storage;

// 구조체 학습 예제는 별도 파일인 structures.rs에 정의한다.
mod structures;

// 변수 선언과 기본 타입 예제는 variables_and_types.rs에 분리한다.
mod variables_and_types;

// `use`를 사용하면 긴 모듈 경로를 현재 스코프로 가져올 수 있다.
// `client`와 `server`는 network.rs에서 `pub mod`로 공개했기 때문에 접근 가능하다.
use network::{client, server};

fn append_role(name: &mut String) {
    name.push_str(" developer");
}

fn main() {
    // String은 대입할 때 소유권이 이동한다.
    let owner = String::from("Rust");
    let moved_owner = owner;
    println!("새 소유자: {moved_owner}");

    // 참조로 빌리면 소유권을 넘기지 않고 값을 사용할 수 있다.
    let mut name = String::from("backend");
    append_role(&mut name);
    println!("가변 빌림 결과: {name}");

    // 클로저는 사용하는 방식에 따라 주변 값을 빌리거나 이동한다.
    let numbers = vec![1, 2, 3];
    let print_numbers = || println!("불변 빌림: {numbers:?}");
    print_numbers();
    println!("클로저 실행 후에도 사용 가능: {numbers:?}");

    // 모듈의 공개 함수를 호출한다.
    // `network::protocol`은 비공개 모듈이므로 main에서는 직접 접근할 수 없다.
    network::print_summary();
    server::connect();
    client::send_data("Hello, module!");

    // `mod.rs` 방식으로 정의한 모듈도 사용하는 쪽에서는 차이가 없다.
    storage::print_summary();
    storage::memory::save("학습 데이터");
    storage::disk::save("학습 노트.md");

    // 구조체 생성, 갱신, 메서드 호출 예제를 실행한다.
    structures::run();

    // let, const와 스칼라·복합 타입 예제를 실행한다.
    variables_and_types::run();
}
