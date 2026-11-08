use std::collections::HashMap;

use axum::{
    Json, Router,
    extract::{Form, Multipart, Path, Query},
    http::{HeaderMap, StatusCode, header},
    routing::{get, post},
};
use axum_extra::{
    TypedHeader,
    headers::{ContentType, UserAgent},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::AppState;

#[derive(Deserialize)]
struct UserQuery {
    id: i32,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct TestBody {
    id: i32,
    name: String,
    age: Option<i32>,
}

#[derive(Deserialize)]
struct UserForm {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct ResultResponse {
    result: UserResult,
}

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
struct HeaderResponse {
    name: String,
    user_agent: String,
}

pub fn users() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello, Users!" }))
        .route(
            "/query",
            get(|Query(query): Query<HashMap<String, String>>| async move {
                format!("query: {query:?}")
            }),
        )
        .route(
            "/query2",
            get(|Query(user): Query<UserQuery>| async move {
                format!("{}: {}", user.id, user.name)
            }),
        )
        .route(
            "/json",
            post(|Json(data): Json<TestBody>| async move { Json(data) }),
        )
        .route(
            "/user-form",
            post(|Form(form): Form<UserForm>| async move {
                Json(ResultResponse {
                    result: UserResult {
                        username: form.name,
                        user_id: form.id,
                    },
                })
            }),
        )
        .route(
            "/user-form-multipart",
            post(|mut body: Multipart| async move {
                let mut file_name = String::new();
                let mut file_size = 0;

                while let Some(field) = body.next_field().await.unwrap() {
                    match field.name().unwrap_or("") {
                        "name" => file_name = field.text().await.unwrap(),
                        "file" => file_size = field.bytes().await.unwrap().len(),
                        _ => {}
                    }
                }

                format!("파일명: {file_name}, 파일 크기: {file_size} bytes")
            }),
        )
        .route(
            "/headers",
            get(
                |headers: HeaderMap, Query(query): Query<NameQuery>| async move {
                    let user_agent = headers
                        .get(header::USER_AGENT)
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("unknown");

                    Json(HeaderResponse {
                        name: query.name,
                        user_agent: user_agent.to_owned(),
                    })
                },
            ),
        )
        .route(
            "/response",
            get(|| async {
                Json(json!({
                    "name": "user",
                    "age": null,
                    "address": [
                        {"city": "Seoul", "state": "Seoul"},
                        {"city": "Busan", "state": "Busan"}
                    ]
                }))
            }),
        )
        .route(
            "/status",
            get(
                |TypedHeader(user_agent): TypedHeader<UserAgent>| async move {
                    println!("user-agent: {}", user_agent.as_str());
                    (
                        StatusCode::OK,
                        TypedHeader(ContentType::text_utf8()),
                        Json(json!({ "status": "success" })),
                    )
                },
            ),
        )
        .route(
            "/{user_id}",
            get(|Path(user_id): Path<String>| async move { format!("user_id: {user_id}") }),
        )
}
