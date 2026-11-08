# rust-basics

Rust의 소유권·빌림·클로저와 모듈 구성을 학습하는 예제 크레이트입니다. 개념 설명은 `docs`에, 실행 가능한 코드는 `src`에 분리했습니다.

## 실행

워크스페이스 루트에서 실행합니다.

```bash
cargo run -p rust-basics
```

컴파일과 테스트만 확인하려면 다음 명령을 사용합니다.

```bash
cargo test -p rust-basics
```

## 구성

```text
rust-basics/
├── docs/
│   ├── module.md
│   └── 소유권과빌림.md
└── src/
    ├── main.rs
    ├── network.rs
    ├── network/
    │   ├── client.rs
    │   ├── protocol.rs
    │   └── server.rs
    └── storage/
        ├── mod.rs
        ├── disk.rs
        └── memory.rs
```

## 학습 자료

| 주제 | 상세 문서 | 실행 예제 |
| --- | --- | --- |
| 소유권, 빌림, 클로저 | [소유권과 빌림](docs/소유권과빌림.md) | [`main.rs`](src/main.rs) |
| 모듈, 경로, 가시성 | [Rust 모듈](docs/module.md) | [`network.rs`](src/network.rs), [`network/`](src/network/) |
| `mod.rs` 방식 | [Rust 모듈](docs/module.md#modrs-방식) | [`storage/mod.rs`](src/storage/mod.rs) |

`README.md`는 프로젝트 탐색과 실행 방법만 안내합니다. 각 개념과 코드 설명은 위 상세 문서 및 소스의 주석을 참고합니다.
