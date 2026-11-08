# rust-axum

데이터베이스와 분리하여 Axum 자체의 라우팅, 요청·응답 추출, 공유 상태, CORS, 핸들러 디버깅을 학습하는 서버입니다.

## 실행

```bash
cargo run -p rust-axum
```

서버는 `http://localhost:3000`에서 실행됩니다.

## Axum 서버의 구성

Axum은 Tokio 비동기 런타임 위에서 동작합니다. `Router`에 경로와 비동기 핸들러를 등록하고, `TcpListener`와 결합해 서버를 실행합니다.

```rust
let app = Router::new().route("/", get(|| async { "Hello, World!" }));
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
axum::serve(listener, app).await?;
```

라우트가 많아지면 기능별 `Router`를 만든 뒤 `nest`로 조합합니다.

```rust
let app = Router::new()
    .nest("/users", users())
    .nest("/state", state())
    .nest("/debug", debug());
```

## 요청 추출과 응답

핸들러 매개변수에서 Extractor를 사용하면 HTTP 요청의 각 부분을 타입으로 안전하게 꺼낼 수 있습니다.

| Extractor | 용도 | 예시 |
| --- | --- | --- |
| `Path<T>` | 경로 파라미터 | `/users/{user_id}` |
| `Query<T>` | 쿼리 문자열 | `/users/query?name=hong` |
| `Json<T>` | JSON 본문 | `Content-Type: application/json` |
| `Form<T>` | URL encoded 폼 | `application/x-www-form-urlencoded` |
| `Multipart` | 파일·텍스트 폼 | `multipart/form-data` |
| `State<T>` | 애플리케이션 공유 상태 | 카운터, 설정, DB 풀 |

응답은 `IntoResponse`를 구현한 문자열, `Json<T>`, `(StatusCode, body)` 등의 형태로 반환할 수 있습니다. `TypedHeader`를 사용하면 특정 헤더도 타입으로 추출하거나 응답에 추가할 수 있습니다.

## 공유 상태

`with_state`로 등록한 값은 요청마다 복제되므로 상태 타입은 `Clone`이어야 합니다. 여러 요청이 같은 가변 데이터를 공유하려면 `Arc<Mutex<T>>` 같은 동기화 타입을 사용합니다.

```rust
#[derive(Clone)]
struct AppState {
    numbers: Arc<Mutex<Vec<i32>>>,
}
```

이 예제의 `GET /state`는 벡터의 첫 값을 안전하게 증가시킵니다. 실제 서비스에서는 동기 함수인 `std::sync::Mutex`의 잠금을 `await` 너머로 유지하지 않도록 주의하고, 비동기 임계 구역이 필요하면 Tokio의 동기화 도구를 고려합니다.

## CORS

`tower-http`의 `CorsLayer`를 모든 라우트 등록 뒤에 추가했습니다. 자격 증명을 허용하는 경우 `*` 대신 명시적인 Origin을 사용해야 합니다.

```rust
let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
    .allow_methods([Method::GET, Method::POST])
    .allow_credentials(true);
```

## 핸들러 디버깅

Axum 핸들러는 `async fn`이어야 하며 결과가 `IntoResponse`로 변환되어야 합니다. 복잡한 타입 오류에는 `macros` 기능과 `#[debug_handler]`를 사용하면 더 직접적인 진단을 얻을 수 있습니다.

## 예제 API

| Method | 경로 | 설명 |
| --- | --- | --- |
| GET | `/` | 기본 문자열 응답 |
| GET | `/users/{user_id}` | 경로 파라미터 추출 |
| GET | `/users/query?name=hong` | 동적 쿼리 추출 |
| GET | `/users/query2?id=1&name=hong` | 구조체 쿼리 추출 |
| POST | `/users/json` | JSON 요청을 JSON으로 응답 |
| POST | `/users/user-form` | URL encoded 폼 처리 |
| POST | `/users/user-form-multipart` | 멀티파트 처리 |
| GET | `/users/headers?name=hong` | 헤더와 쿼리 추출 |
| GET | `/users/status` | 상태 코드와 Typed Header 응답 |
| GET | `/state` | 공유 카운터 증가 |
| GET | `/debug` | `debug_handler` 예제 |

## 상세 학습 노트

- [라우팅과 핸들러](docs/route.md)
- [공유 상태](docs/state.md)
- [CORS](docs/cors.md)
- [핸들러 디버깅](docs/handler_debugging.md)
