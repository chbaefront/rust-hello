# route.md

---

### 라우트 정의

---
`Router::new()`를 사용하여 라우터 인스턴스 생성   
해당 인스턴스에 `route()` 함수를 연결하여 경로와 핸들러를 연결하여 전체 라우터를 완성

```rust
use axum::{Router, routing::{ get, post, put, delete }};

let app = Router::new()
.route("/get", get(|| async { "Hello, Axum!" }))
.route("/post", post(|| async { "Hello, Axum!" }))
.route("/put", put(|| async { "Hello, Axum!" } ))
.route("/delete", delete(|| async { "Hello, Axum!" }));

```

`|| async { }` 이 구문은 러스트에서 클로저 또는 익명 함수라고 부릅니다.

route()는 인자로 url 경로 문자열과 처리할 라우팅 함수를 받습니다.

```rust
// 인자가 없는 클로저
|| {
"Hello"
}

// 또는 한줄로.
|| "Hello"

let handler = || "Hello";
let result = handler();

// 인자가 있는 클로저
|name: &str| {
format!("Hello, {name}!")
}

let greeting = |name: &str| format!("Hello, {name}!");
println!("{}", greeting("Alice"));
```

라우트를 route()로 체이닝 하면 길어지고 관리가 어렵기 때문에 라우트를 합치는 `nest()`를 사용   
nest()는 앞에 지정한 경로를 하위 라우터의 모든 경로 앞에 붙입니다.
```rust
use axum::Router;
mod routes;

let app = Router::new()
        .nest("/users", routes::users::router())
        .nest("/posts", routes::posts::router());
```
```rust
// users.rs
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_users))
        .route("/", post(create_user))
}

async fn list_users() -> &'static str {
    "user list"
}

async fn create_user() -> &'static str {
    "create user"
}
```

### 핸들러

---
라우터를 정의하면서 get, post와 같은 함수의 두 번째 인자로 클로저를 전달했습니다. 이를 핸들러 함수라고 합니다.   
각 요청을 처리하므로 핸들러 함수 입니다.   

라우터의 각 경로에 등록되는 핸들러 함수는 반드시 async 키워드를 사용해서 만든 **비동기 함수**여야 합니다.

```rust
route("/users", || async { "Hello, Handler!" });
```

Axum의 핸들러 함수는 최종적으로 IntoResponse로 변환될 수 있는 값을 반환

### 핸들러(경로 파라미터, 쿼리 파라미터, 요청 본문 값 추출)

---
- 경로 파라미터   
경로 파라미터는 url에서 `{파라미터명}` 으로 표현
```
/users/{user_id}
```

```rust
use axum::{Router, extract::Path, routing::get};

pub fn users() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, Users!!" }))
        .route("/{user_id}", get(|Path(user_id): Path<String>| async move { format!("user_id: {}", user_id) }))
}
```
`extract::Path`를 통해 경로 파라미터를 지정한 타입으로 변환해 사용 가능   
`async move`는 바깥 변수를 소유권을 가져와서 사용하도록 합니다.

- 쿼리 파라미터   
url에 키-값 쌍에 대한 값을 쿼리 파라미터라고 합니다.
```
/users/query?name=hong&age=20
```
`extract::Query`를 통해 경로 파라미터를 지정한 타입으로 변환해 사용 가능

```rust
use std::collections::HashMap;
use axum::{Router, extract::Path, routing::get};
use axum::extract::Query;

pub fn users() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, Users!!" }))
        .route("/{user_id}", get(|Path(user_id): Path<String>| async move { format!("user_id: {}", user_id) }))
        .route("/query", get(|Query(query): Query<HashMap<String, String>>| async move {
            // println!("query: {}", query["name"]); // name 키가 없으면 에러 발생
            match query.get("name") {
                Some(name) => println!("query(name): {name}"),
                None => println!("name 쿼리가 없습니다."),
            }
            format!("query: {:?}", query)
        }))
}
```
해시맵을 사용하여 키값 쌍을 표현합니다.   
해시맵을 사용해 쿼리 파라미터를 받게 되면 문자열이 아닌 타입을 문자열 타입과 동시에 처리할 수 없습니다.(타입 변환으로 처리)   
만일 쿼리 파라미터 중 하나라도 값이 없다면 에러가 발생(그래서 해시맵보다는 별도의 구조체 타입을 선언해서 사용 + serde::Deserialize 트레이트를 구현)   

https://crates.io/crates/serde

```rust
// 구조체
#[derive(serde::Deserialize)]
struct User {
    id: i32,
    name: String,
}

route("/query2", get(|Query(user): Query<User>| async move{
format!("{}: {}", user.id, user.name)
})) // 이것도 name 쿼리 파라미터를 보내지 않으면 에러 발생
```

- 요청 본문 값 추출   
JSON, form data

`extract::Json`을 사용하여 JSON 형태의 값 추출

```rust
// 요청 JSON 구조체 정의
#[derive(serde::Deserialize, Serialize)]
struct Test {
    id: i32,
    name: String,
    age: Option<i32>
}

route("/json", get(|Json(data): Json<Test>| async move {
           Json(data) // 그대로 받아, Json 으로 응답
        }))
```

`extract::Form`을 사용하여 Form 데이터 추출
```rust
#[derive(serde::Deserialize)]
struct UserForm {
    id: i32,
    name: String,
}

// 응답용: Json 결과의 바깥 껍데기
#[derive(Serialize)]
struct ResultResponse {
    result: UserResult,
}

// 응답용: 데이터 알맹이
#[derive(Serialize)]
struct UserResult {
    user_id: String,
    username: String,
}

route(
"/user-form",
post(|Form(form): Form<UserForm>| async move {
Json(ResultResponse {
result: UserResult {
username: form.name,
user_id: form.id.to_string(),
},
})
}),
)
```

multipart 는 `extract::Multipart` 를 사용
```rust
// Cargo.toml 을 가서, 아래와 같이 수정(멀티 파트 피처를 추가
axum = { version = "0.8.9", features = ["json", "multipart"]}
```

```rust

.route(
            "/user-form-multipart",
            post(|mut body: Multipart| async move {
                let mut file_name = String::new();
                let mut file_data = Vec::new();

                // 폼 데이터를 한 칸씩 순회하면서 검사합니다
                // next_field().await로 필드를 하나씩 추출
                while let Some(field) = body.next_field().await.unwrap() {
                    let field_name = field.name().unwrap_or("").to_string();

                    match field_name.as_str() {
                        "name" => {
                            // 텍스트 필드 읽기
                            file_name = field.text().await.unwrap();
                        }
                        "file" => {
                            // 파일 필드 읽기
                            file_data = field.bytes().await.unwrap().to_vec();
                        }
                        _ => {
                            // 모르는 필드는 무시
                        }
                    }
                }

                format!(
                    "파일명: {}, 파일 크기: {} bytes",
                    file_name,
                    file_data.len()
                )
            }),
        )
```

body값이 next_field가 호출될 때마다 변경되기 때문에 가변 소유권이 필요

