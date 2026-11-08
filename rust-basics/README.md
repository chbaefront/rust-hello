# rust-basics

Rust 언어의 핵심인 소유권·빌림·클로저와 모듈 구성을 학습하는 프로젝트입니다.

## 실행

워크스페이스 루트에서 실행합니다.

```bash
cargo run -p rust-basics
```

## 소유권과 빌림

Rust의 각 값에는 한 명의 소유자가 있습니다. 소유자가 스코프를 벗어나면 값은 자동으로 해제됩니다. `String`, `Vec`처럼 힙 데이터를 가진 값은 다른 변수나 함수에 값 자체를 전달하면 소유권이 이동합니다.

```rust
let first = String::from("hello");
let second = first;
println!("{second}");
// println!("{first}"); // 소유권이 이동했으므로 컴파일 오류
```

값을 계속 사용해야 한다면 참조로 빌립니다. 여러 불변 참조는 허용되지만, 가변 참조가 존재하는 동안에는 다른 참조를 동시에 사용할 수 없습니다.

```rust
fn append_role(value: &mut String) {
    value.push_str(" developer");
}

let mut name = String::from("backend");
append_role(&mut name);
println!("{name}");
```

`i32`, `bool`, `char` 같은 `Copy` 타입은 대입하거나 함수에 전달할 때 값이 복사되므로 기존 변수도 계속 사용할 수 있습니다.

## 클로저와 캡처

클로저의 매개변수에는 일반 함수와 같은 소유권 규칙이 적용됩니다. 주변 변수를 캡처하는 방식에 따라 관련 트레이트가 달라집니다.

| 캡처 방식 | 대표 트레이트 | 예시 |
| --- | --- | --- |
| 불변 빌림 | `Fn` | 값을 읽거나 출력 |
| 가변 빌림 | `FnMut` | 카운터 증가, 벡터 변경 |
| 소유권 이동 | `FnOnce` | `move` 클로저, 값 소비 |

## 모듈과 크레이트

- `mod`는 모듈을 정의하거나 모듈 파일을 선언합니다.
- `pub`은 외부 모듈에 항목을 공개합니다.
- `use`는 긴 모듈 경로를 현재 스코프로 가져옵니다.
- `main.rs`는 바이너리 크레이트, `lib.rs`는 라이브러리 크레이트의 루트입니다.

```rust
mod network {
    pub mod client {
        pub fn connect() {}
    }
}

fn main() {
    network::client::connect();
}
```

파일 기반 모듈은 `mod routes;` 선언과 `routes.rs` 또는 `routes/mod.rs` 파일을 연결합니다. 라이브러리 모듈을 바이너리에서 사용할 때는 패키지 이름의 하이픈을 밑줄로 바꾼 크레이트 경로를 사용합니다.

## 상세 학습 노트

- [소유권과 빌림](docs/소유권과빌림.md)
- [Rust 모듈](docs/module.md)
