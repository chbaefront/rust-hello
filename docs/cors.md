# cors.md

---

### Axum에서 CORS 허용해주기

---

- Axum에서는 일반적으로 `tower-http`의 `CorsLayer`를 사용
```
cargo add tower-http
```

```
tower-http = { version = "0.7.1", features = ["cors"]}
```

```rust
use tower_http::cors::{Any, CorsLayer};

let cors = CorsLayer::new()
.allow_origin(Any) // 모든 도메인 허용 = Any
.allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
.allow_headers(vec![header::AUTHORIZATION, header::ACCEPT])
.allow_credentials(true);
```

`allow_credentials(true)` 인 경우에는 구체적인 Origin을 지정해야한다.

`allow_origin()` 은 `Into<AllowOrigin>` 타입을 인자로 받습니다.

```rust
// HeaderValue는 Into<AllowOrigin>을 구현함
allow_origin(
    HeaderValue::from_static("http://localhost:5173")
)

// 여러 Origin 허용
allow_origin([
HeaderValue::from_static("http://localhost:5173"),
HeaderValue::from_static("http://localhost:3000"),
])
```

HeaderValue 타입을 받아서, AllowOrigin 타입으로 변환을 하는 것 같다.

```rust
let app = Router::new()
        .route("/", get(callback))
        .nest("/users", users())
        .nest("/state", state())
        .nest("/debug", debug())
        .layer(cors)
        .with_state(data);
```

생성한 cors 레이어 인스턴스는 `layer()` 메서드에 등록

주의할 점: `layer(cors)`는 호출 시점에 이미 등록된 라우트에 적용됩니다.  
따라서 전체 API에 CORS를 적용하려면 지금처럼 모든 `route()`와 `nest()`를 등록한 뒤에 호출하면 됩니다.