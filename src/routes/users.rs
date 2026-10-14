use axum::http::{HeaderMap, header};
use axum::{
    Router,
    extract::{Form, Json, Multipart, Path, Query},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

// 구조체
#[derive(serde::Deserialize)]
struct User {
    id: i32,
    name: String,
}

// 요청 JSON 구조체 정의
#[derive(serde::Deserialize, Serialize)]
struct Test {
    id: i32,
    name: String,
    age: Option<i32>,
}

#[derive(serde::Deserialize)]
struct UserForm {
    id: String,
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

#[derive(Deserialize)]
struct NameQuery {
    name: String,
}

#[derive(Serialize)]
struct Response {
    name: String,
    user_agent: String,
}

#[derive(Serialize)]
struct JsonResponse {
    name: String,
    age: Option<i32>,
    address: Vec<Address>,
}

#[derive(Serialize)]
struct Address {
    city: String,
    state: String,
}

pub fn users() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, Users!!" }))
        .route(
            "/{user_id}",
            get(|Path(user_id): Path<String>| async move { format!("user_id: {}", user_id) }),
        )
        .route(
            "/query",
            get(|Query(query): Query<HashMap<String, String>>| async move {
                // println!("query: {}", query["name"]); // name 키가 없으면 에러 발생
                match query.get("name") {
                    Some(name) => println!("query(name): {name}"),
                    None => println!("name 쿼리가 없습니다."),
                }
                format!("query: {:?}", query)
            }),
        )
        .route(
            "/query2",
            get(|Query(user): Query<User>| async move { format!("{}: {}", user.id, user.name) }),
        )
        .route(
            "/json",
            get(|Json(data): Json<Test>| async move {
                // age 는 없으면 null
                Json(data) // 그대로 받아, Json 으로 응답
            }),
        )
        .route(
            "/user-form",
            post(|Form(form): Form<UserForm>| async move {
                // x-www-form-urlencoded
                Json(ResultResponse {
                    result: UserResult {
                        username: form.name,
                        user_id: form.id.to_string(),
                    },
                })
            }),
        )
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
        .route(
            "/headers",
            get(
                |headers: HeaderMap, Query(query): Query<NameQuery>| async move {
                    let user_agent = headers
                        .get(header::USER_AGENT)
                        .and_then(|v| v.to_str().ok()) // HeaderValue가 있고 올바른 문자열이면 &str로 변환
                        .unwrap_or("user_agent"); // 없거나 변환 실패 시 기본 &str 사용

                    let name = query.name.to_string();

                    // user_agent.to_string() // 최종적으로 String 반환
                    Json(Response {
                        name,
                        user_agent: user_agent.to_string(),
                    })
                },
            ),
        )
        .route(
            "/response",
            get(|| async move {
                // json 응답
                // 구조체 정의 후,#[derive(Serialize)] json 직렬화
                // JsonResponse 구조체와 Address 구조체..실제로 작성하니 구조체를 표현하는게 번거롭다.
                // Json(
                //     JsonResponse {
                //         name: "홍길동".to_string(),
                //         age: Option::from(10),
                //         address: vec!(Address {
                //             state: "z".to_string(),
                //             city: "z".to_string()
                //         }, Address {
                //             state: "x".to_string(),
                //             city: "x".to_string()
                //         }),
                //     }
                // )

                // json 매크로 사용
                // serde_json = "1.0.154" 추가
                // 이 방법은 간단하지만, 매번 동적으로 값을 만들어내기 때문에 성능이 느려질 수 있는 단점이 존재 + 컴파일 타임에 타입이 정해지지 않음
                let result = json!({
                    "name": "user",
                    "age": null,
                    "address": [
                          {"city": "s", "state": "s"},
                          {"city": "c", "state": "s"}
                    ]
                });

                Json(result)
            }),
        )
}
