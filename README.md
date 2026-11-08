# Rust 학습 워크스페이스

하나의 프로젝트에 섞여 있던 예제를 개념별 Cargo 패키지로 분리한 워크스페이스입니다. 각 패키지는 독립적으로 학습하고 실행할 수 있습니다.

## 프로젝트 구성

| 경로 | 학습 주제 | 실행 |
| --- | --- | --- |
| [`rust-basics`](rust-basics/README.md) | 소유권, 빌림, 클로저, 모듈 | `cargo run -p rust-basics` |
| [`rust-axum`](rust-axum/README.md) | Axum 라우팅, Extractor, State, CORS, 핸들러 디버깅 | `cargo run -p rust-axum` |
| [`rust-axum-orm`](rust-axum-orm/README.md) | PostgreSQL, SeaORM, SeaQuery, 엔티티, 마이그레이션 | `cargo run -p rust-axum-orm` |

## 공통 명령어

```bash
# 전체 패키지 컴파일 검사
cargo check --workspace

# 특정 패키지 검사 또는 실행
cargo check -p rust-axum
cargo run -p rust-axum

# 전체 테스트
cargo test --workspace
```

빌드 결과물과 `Cargo.lock`은 워크스페이스 루트에서 공통으로 관리합니다. 데이터베이스 연결 정보가 담긴 `.env`는 Git에 포함되지 않습니다.

## 개발 환경

- IDE: RustRover
- Rust edition: 2024
- 웹 프레임워크: Axum
- ORM: SeaORM
