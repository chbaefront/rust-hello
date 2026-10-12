# rust-hello

---

- IDE: RustRover
- rust: 1.99.0   
- cargo: 1.99.0

### 프로젝트 생성

```
cargo new rust-hello
```

### 실행 방법

- rust 설치 후, `cargo`를 통해 실행
```
cargo run
```

- rust도 C언어와 마찬가지로 `fn main()` 함수가 Entry Point역할을 합니다.

- 빌드 및 컴파일 상태 확인
```
cargo build
cargo build --release
cargo check
```

빌드 결과물은 `target/debug` 또는 `target/release` 폴더를 확인