use axum::{
    Router,
    extract::{Form, Json, Multipart, Path, Query},
    routing::{get, post},
};
use serde::Serialize;
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
}
