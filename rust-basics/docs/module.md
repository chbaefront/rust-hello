# Rust 모듈

Rust의 모듈 시스템은 코드를 논리적인 단위로 나누고, 경로와 가시성을 통해 각 항목의 접근 범위를 관리합니다.

## 기본 용어

- **패키지(package)**: 하나의 `Cargo.toml`로 관리되는 프로젝트입니다.
- **크레이트(crate)**: Rust 컴파일 단위입니다. 바이너리 크레이트와 라이브러리 크레이트가 있습니다.
- **모듈(module)**: 크레이트 내부의 코드를 계층적으로 구성하는 단위입니다.
- **경로(path)**: `network::client::send_data`처럼 항목의 위치를 나타냅니다.

하나의 패키지는 여러 바이너리 크레이트와 최대 하나의 라이브러리 크레이트를 가질 수 있습니다. `main.rs`는 기본 바이너리 크레이트의 루트이고 `lib.rs`는 라이브러리 크레이트의 루트입니다.

## 인라인 모듈 정의

중괄호를 사용하면 현재 파일 안에 모듈을 직접 정의할 수 있습니다.

```rust
mod greeting {
    pub fn say_hello(name: &str) {
        println!("안녕하세요, {name}!");
    }
}

fn main() {
    greeting::say_hello("Rust");
}
```

모듈과 모듈 안의 항목은 기본적으로 비공개입니다. 부모 모듈 밖에서 사용해야 하는 항목에는 `pub`을 붙입니다.

## 파일 기반 모듈 선언

중괄호 없이 `mod network;`라고 작성하면 현재 파일에 코드를 정의하는 대신 컴파일러가 모듈 파일을 찾습니다.

```rust
// main.rs
mod network;

fn main() {
    network::server::connect();
}
```

`main.rs`와 같은 디렉터리에서 다음 두 형태 중 하나를 사용할 수 있습니다.

1. `network.rs`
2. `network/mod.rs`

두 파일을 동시에 만들면 같은 모듈의 후보가 중복되므로 컴파일 오류가 발생합니다.

## `모듈명.rs` 방식

이 프로젝트의 `network` 모듈은 `network.rs`를 루트로 사용합니다.

```text
src/
├── main.rs
├── network.rs
└── network/
    ├── client.rs
    ├── protocol.rs
    └── server.rs
```

```rust
// network.rs
pub mod client;
pub mod server;
mod protocol;

pub fn print_summary() {
    println!("네트워크 모듈");
}
```

`network.rs`의 `mod client;` 선언은 `network/client.rs` 또는 `network/client/mod.rs`에서 자식 모듈을 찾습니다. 이 프로젝트에서는 전자를 사용합니다.

이 방식은 모듈의 루트 파일을 `src`에서 바로 확인할 수 있어 최근 프로젝트에서 흔히 사용합니다.

## `mod.rs` 방식

`storage` 모듈은 별도의 `storage.rs` 없이 `storage/mod.rs`를 루트로 사용합니다.

```text
src/
├── main.rs
└── storage/
    ├── mod.rs
    ├── disk.rs
    └── memory.rs
```

```rust
// main.rs
mod storage;

fn main() {
    storage::print_summary();
    storage::memory::save("학습 데이터");
    storage::disk::save("학습 노트.md");
}
```

```rust
// storage/mod.rs
pub mod disk;
pub mod memory;

pub fn print_summary() {
    println!("storage/mod.rs 방식");
}
```

`mod.rs` 방식은 모듈 루트와 자식 파일을 한 디렉터리에 모을 수 있습니다. 기존 Rust 프로젝트에서 자주 볼 수 있으며 현재 Rust에서도 지원됩니다.

| 모듈 | 루트 파일 | 자식 모듈 예시 |
| --- | --- | --- |
| `network` | `network.rs` | `network/client.rs` |
| `storage` | `storage/mod.rs` | `storage/memory.rs` |

파일 배치는 다르지만 호출하는 코드에서는 두 방식에 차이가 없습니다.

## 가시성

```rust
pub mod client;     // 부모 모듈 밖에도 공개
mod protocol;       // 현재 모듈과 그 자식에서만 접근 가능

pub fn connect() {} // 공개 함수

pub(super) fn address() {
    // 부모 모듈 범위까지만 공개
}
```

- `pub`: 접근 가능한 외부 범위에 항목을 공개합니다.
- `pub(crate)`: 현재 크레이트 전체에 공개합니다.
- `pub(super)`: 부모 모듈 범위에 공개합니다.
- `pub(in crate::some_module)`: 지정한 조상 모듈 범위에 공개합니다.

부모 모듈의 비공개 항목은 그 부모와 자식 모듈에서 사용할 수 있지만, 부모 바깥에서는 접근할 수 없습니다. 반대로 자식 모듈에 있는 항목은 부모가 사용하려는 경우에도 적절한 가시성이 필요합니다.

## `crate`, `self`, `super`

모듈 경로는 다음 키워드로 시작할 수 있습니다.

- `crate`: 현재 크레이트의 루트
- `self`: 현재 모듈
- `super`: 부모 모듈

```rust
// network/client.rs
use super::server;

pub fn send_data(message: &str) {
    println!("{}에 전송: {message}", server::address());
}
```

`client`와 `server`는 형제 모듈이므로 `client`에서 `super::server` 경로로 접근합니다.

## `use`로 경로 가져오기

`use`는 모듈을 새로 정의하지 않습니다. 기존 항목의 경로를 현재 스코프로 가져와 짧게 사용할 수 있게 합니다.

```rust
mod network; // network 모듈을 모듈 트리에 등록

use network::{client, server}; // 등록된 모듈의 경로를 현재 스코프로 가져옴

fn main() {
    server::connect();
    client::send_data("Hello, module!");
}
```

`use`로 가져온 이름을 다른 모듈에서도 사용하게 하려면 `pub use`로 다시 공개할 수 있습니다. 이를 재노출(re-export)이라고 합니다.

## `lib.rs`와 `main.rs`

`lib.rs`가 패키지를 별도의 패키지로 바꾸는 것은 아닙니다. 같은 패키지 안에 라이브러리 크레이트를 추가합니다. `main.rs`의 바이너리 크레이트는 라이브러리 크레이트를 외부 크레이트처럼 이름으로 참조할 수 있습니다.

```text
src/
├── lib.rs
├── main.rs
└── routes/
    └── users.rs
```

```rust
// lib.rs
pub mod routes {
    pub mod users;
}
```

```rust
// main.rs
use my_app::routes::users::users_router;

fn main() {
    users_router();
}
```

패키지 이름에 하이픈이 있으면 Rust 코드의 크레이트 경로에서는 밑줄을 사용합니다. 예를 들어 패키지 `rust-basics`의 크레이트 이름은 `rust_basics`입니다.
